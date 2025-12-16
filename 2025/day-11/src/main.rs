use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A label for a vertex in a graph.
type Label = &'static str;

/// A graph represented as a set of adjacency lists.
type Graph = HashMap<Label, Vec<Label>>;

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut graph = Graph::default();
    for line in input.lines() {
        let (source, rest) = line.split_once(':').expect("puzzle input lines should contain a colon");
        let neighbours = rest.split_whitespace();
        let adj_list = graph.entry(source).or_default();
        adj_list.extend(neighbours);
    }

    let mut searcher = Searcher::new(&graph);
    let num_you_out = searcher.count_simple_paths("you", "out", State::already_valid());
    let num_svr_out = searcher.count_simple_paths("svr", "out", State::new());

    println!("Number of paths from 'you' to 'out' (part 1): {num_you_out}");
    println!("Number of paths from 'svr' to 'out' which visit 'fft' and 'dac' (part 2): {num_svr_out}");
}

// This searcher-based approach is much more involved than it needs to be. But it was a good exercise in writing
// something general purpose that could be expanded to more types of searches than just the `fft`/`dac` thing. For
// example, we could implement `SearchState` on `()` for non-stateful searches. Maybe it'd be worth pulling it out into
// `aoc_utils` at some point.

/// A memoized, stateful graph searcher.
///
/// This struct counts the number of paths that meet some condition, represented by some [state][SearchState].
///
/// Using a single searcher for multiple searches allows re-using the memoized cache across searches.
struct Searcher<'a, S: SearchState> {
    graph: &'a Graph,
    memo: HashMap<(Label, S), usize>,
    path: HashSet<(Label, S)>,
}

/// Represents the current state of a search through a graph.
trait SearchState: Copy {
    /// Gets an updated copy of the current state.
    fn update(self, node: Label) -> Self;

    /// Determines if the search with this state counts as a valid path.
    fn is_valid(&self) -> bool;
}

impl<'a, S: SearchState> Searcher<'a, S> {
    pub fn new(graph: &'a Graph) -> Searcher<'a, S> {
        Self {
            graph,
            memo: HashMap::default(),
            path: HashSet::default(),
        }
    }
}

impl<'a, S: SearchState> Searcher<'a, S>
where
    S: Eq + Hash,
{
    /// Finds the number of simple paths between two vertices in the searcher's graph.
    pub fn count_simple_paths(&mut self, src: Label, dst: Label, mut state: S) -> usize {
        // In a graph, the number of paths *A→(B,C)→(...)→D* is equal to the number of paths from *B→D* plus the number
        // of paths from *C→D*. However, those paths from *B* and *C* might be very similar, or even completely
        // identical! We don't want to have to do all that searching twice. So, we'll do the DFS scan from *B→(...)→D*
        // as normal, but for every node *N* in between, we'll store the number of *N→(...)→D* paths. Then, if/when
        // *C→(...)→D* encounters *N*, it doesn't need to travel down that whole leg again.
        //
        // To handle the case of a stateful search (e.g., "have we seen `fft` and `dac` yet?") we include that state in
        // the cache. For example, consider: *A→(B,C)→D→E* when searching for *A→E*, where we wish to encounter node
        // *B*. When we get to *E* through *B* the first time, we want to write down the *D* was part of 1 valid path.
        // But if we just write down *D,1*, then the second branch of the search, through *A→C→D*, will think that the
        // *D* should be short-circuited with a 1; but it should really be zero.
        if self.path.contains(&(src, state)) {
            // If we have already visited this node along the way, we have encountered a cycle. That wouldn't be a
            // simple path, so we do not count it. That is, however, unless we have since encountered something which
            // has updated the current state: did we see fft or dac? If so, keep scanning (handled by the fact that we
            // use `state` in the key).
            return 0;
        } else if let Some(&count) = self.memo.get(&(src, state)) {
            // If we already know how many paths go from this node to the destination, just return that.
            count
        } else {
            state = state.update(src);
            if src == dst {
                // If we are trying to go from node A to node A, there is exactly one path: []. That is, as long as we
                // exclude all the possible A->A cycles. Which we do, because we're counting simple paths. Also, for
                // part 2, we only count it when the state deems to be valid.
                let count = if state.is_valid() { 1 } else { 0 };
                self.memo.insert((src, state), count);
                count
            } else {
                // Otherwise, we need to iterate all neighbours.
                let Some(neighbours) = self.graph.get(&src) else {
                    // If the node we're trying to search from isn't in the graph, there are no paths.
                    return 0;
                };

                self.path.insert((src, state));

                let mut count = 0;
                for &neighbour in neighbours {
                    count += self.count_simple_paths(neighbour, dst, state);
                }

                self.path.remove(&(src, state));
                self.memo.insert((src, state), count);
                count
            }
        }
    }
}

/// The [search state][SearchState] for this particular Advent of Code problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    seen_fft: bool,
    seen_dac: bool,
}

impl State {
    /// Creates a new search state that has not yet seen `fft` nor `dac`.
    pub const fn new() -> Self {
        Self { seen_fft: false, seen_dac: false }
    }

    /// Creates a new search state that has already seen `fft` and `dac`.
    pub const fn already_valid() -> Self {
        Self { seen_fft: true, seen_dac: true }
    }
}

impl SearchState for State {
    fn update(mut self, node: Label) -> State {
        self.seen_fft |= node == "fft";
        self.seen_dac |= node == "dac";
        self
    }

    fn is_valid(&self) -> bool {
        self.seen_fft && self.seen_dac
    }
}
