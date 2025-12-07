use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Condvar, Mutex};

use aoc_utils::grid::{Dir4, Grid, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Splitter,
}

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut start_pos = None;
    let map = Grid::from_lines_map(input.lines(), |c, pos| match c {
        '.' => Tile::Empty,
        '^' => Tile::Splitter,
        'S' if start_pos.is_none() => {
            start_pos = Some(pos);
            Tile::Empty
        },
        'S' => panic!("invalid puzzle input: multiple 'S' chars"),
        _ => panic!("invalid puzzle input: unknown char {c}"),
    })
    .unwrap();
    let start_pos = start_pos.expect("puzzle input should contain one 'S' character for starting position");

    if aoc_utils::verbosity() > 1 {
        println!("{map:?}");
    }

    // We'll use a threadpool to handle the branching paths.
    let mut pool = aoc_utils::threadpool();
    let mut beam_queue = BeamQueue::new(start_pos);

    pool.scoped(|scope| {
        let map = &map;
        while let Some(pos) = beam_queue.dequeue() {
            if aoc_utils::verbosity() > 0 {
                println!("\tTachyon beam was spawned at position {pos:2?}.");
            }

            let queue = beam_queue.clone();
            scope.execute(move || tachyon_beam(map, pos, queue));
        }
    });

    let num_splits = beam_queue.finish();
    println!("Total number of tachyon beam splits (part 1): {num_splits}");
}

fn tachyon_beam(map: &Grid<Tile>, mut pos: Pos, mut beam_queue: BeamQueue) {
    while map.contains(pos) {
        match map[pos] {
            Tile::Empty => pos += Dir4::Down,
            Tile::Splitter => {
                if aoc_utils::verbosity() > 0 {
                    println!("Tachyon beam was split at position {pos:2?}.");
                }

                beam_queue.enqueue_split(map, pos);
                break;
            },
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Splitter => "^",
        })
    }
}

/// A work-queue for beams that need to be simulated.
///
/// This struct is designed to mimic the behaviour of [`mpsc::channel`], with one key difference: it does not require
/// the last "`Sender`" to be dropped before its "`recv`" method unblocks. Instead, [`dequeue`] will unblock and return
/// `None` when there are no **other** [`BeamQueue`] instances remaining alive.
///
/// [`mpsc::channel`]: std::sync::mpsc::channel
/// [`dequeue`]: BeamQueue::dequeue
#[derive(Debug, Clone)]
struct BeamQueue {
    inner: Arc<QueueInner>,
}

#[derive(Debug)]
struct QueueInner {
    cvar: Condvar,
    data: Mutex<QueueData>,
}

#[derive(Debug)]
struct QueueData {
    num_splits: usize,
    beam_queue: VecDeque<Pos>,
}

impl BeamQueue {
    pub fn new(initial: Pos) -> Self {
        Self {
            inner: Arc::new(QueueInner {
                cvar: Condvar::new(),
                data: Mutex::new(QueueData {
                    num_splits: 0,
                    beam_queue: vec![initial].into(),
                }),
            }),
        }
    }

    /// Gets the next beam from the queue.
    ///
    /// This method will block the current thread until either (a) another beam appears in the queue, or (b) all other
    /// [`BeamQueue`] instances have been dropped.
    pub fn dequeue(&mut self) -> Option<Pos> {
        // - Lock the mutex
        // - Check how many things are in the queue; if there's something there, return it.
        // - If there is nothing, check the reference count:
        //   - If 1, we are the last BeamQueue alive, and we should return `None`.
        //   - Otherwise, there's another BeamQueue out there which may push things onto the queue, so we should wait.
        // Once we've received a condvar notification, then another queue has either (a) pushed something or (b) been
        // dropped. In either case, we go back to step 2 and try to dequeue a second time; if there are still other
        // BeamQueues out there, we wait again, and so on.
        let mut guard = self.inner.data.lock().expect("lock was poisoned");
        loop {
            if let Some(pos) = guard.beam_queue.pop_front() {
                break Some(pos);
            } else if Arc::strong_count(&self.inner) == 1 {
                break None;
            } else {
                guard = self.inner.cvar.wait(guard).expect("lock was poisoned");
            }
        }
    }

    /// Adds two more beams to the queue.
    pub fn enqueue_split(&mut self, map: &Grid<Tile>, pos: Pos) {
        let mut inner = self.inner.data.lock().expect("lock was poisoned");

        inner.num_splits += 1;

        if map.has_neighbour(pos, Dir4::Left) {
            inner.beam_queue.push_back(pos + Dir4::Left);
        }

        if map.has_neighbour(pos, Dir4::Right) {
            inner.beam_queue.push_back(pos + Dir4::Right);
        }

        // Any queues which are currently trying to dequeue should wake up and try again.
        // [NOTE] notify_one vs. notify_all: https://stackoverflow.com/q/9015748/10549827#comment11304280_9015781
        self.inner.cvar.notify_all();
    }

    /// Deconstructs this BeamQueue's locks and reference counters, returning the final `num_splits`.
    pub fn finish(self) -> usize {
        // We cannot move out of `self` because it implements Drop. To fully extract the inner value, we first need to
        // (1) get our own, not-inside-a-`Self` copy of the Arc, then (2) let self's destructor run so that our Arc is
        // the last one. Then we can pull the Arc's value out onto the stack.
        let inner = Arc::clone(&self.inner);
        drop(self);
        let inner = Arc::into_inner(inner).expect("there should be exactly one BeamQueue remaining");
        let inner = Mutex::into_inner(inner.data).expect("lock was poisoned");
        inner.num_splits
    }
}

impl Drop for BeamQueue {
    fn drop(&mut self) {
        // When the queue is dropped, make sure any awaiting queues check the queue again.
        self.inner.cvar.notify_all();
    }
}
