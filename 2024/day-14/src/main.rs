mod charsets;
mod robot;

use std::io::{self, StdoutLock, Write};
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::process::ExitCode;
use std::time::Duration;

use aoc_utils::Grid;
use crossterm::QueueableCommand;
use crossterm::cursor::{DisableBlinking, EnableBlinking, Hide as HideCursor, MoveTo, Show as ShowCursor};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
#[cfg(not(windows))]
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use self::charsets::CharSet;
use self::robot::{Robot, Vec2};

pub const MAP_W: usize = 101;
pub const MAP_H: usize = 103;

fn main() -> ExitCode {
    let robots = aoc_utils::puzzle_input().lines().map(|line| line.parse().unwrap());

    // Swap `Braille` for `Quadrants` to use different Unicode characters for rendering.
    match App::<charsets::Braille>::new(robots).run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        },
    }
}

pub fn setup_terminal() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.queue(EnterAlternateScreen)?.queue(DisableBlinking)?.queue(HideCursor)?;
    #[cfg(not(windows))]
    // Non-Windows devices need `REPORT_EVENT_TYPES` active to be able to read whether or not a KeyEvent is a
    // press/release/hold etc. But trying to set the flag on Windows results in a crash.
    stdout.queue(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
    stdout.flush()
}

pub fn restore_terminal() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.queue(LeaveAlternateScreen)?.queue(EnableBlinking)?.queue(ShowCursor)?;
    #[cfg(not(windows))]
    stdout.queue(PopKeyboardEnhancementFlags)?;
    stdout.flush()
}

struct App<C: CharSet> {
    robots: Vec<Robot>,
    counts: Grid<u8>,
    safety: u32,
    quad_counts: [u32; 4],
    non_quad_count: u32,
    safety100: Option<u32>,
    timestamp: i32,
    paused: bool,
    _marker: PhantomData<C>,
}

impl<C: CharSet> App<C> {
    pub fn new(robots: impl IntoIterator<Item = Robot>) -> Self {
        let mut counts = Grid::<u8>::empty(MAP_W, MAP_H);
        let robots = robots.into_iter().inspect(|Robot { pos, .. }| counts[*pos] += 1).collect();
        App {
            counts,
            robots,
            safety: 0,
            quad_counts: [0; 4],
            non_quad_count: 0,
            safety100: None,
            timestamp: 0,
            paused: false,
            _marker: PhantomData,
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        setup_terminal()?;

        self.draw()?;

        loop {
            self.recalculate_stats();
            self.draw()?;

            // Images seem to appear when the "safety factor" is below a certain amount; use this to control how long to
            // block for when polling for keyboard events, to create a visible delay for interesting patterns.
            let delay = if self.safety < 200_000_000 {
                Duration::from_millis(100)
            } else {
                Duration::from_millis(0)
            };

            if let ControlFlow::Break(_) = self.handle_events(delay)? {
                break;
            }

            // When it's _really_ low (relatively speaking), there's a high chance that we have found our image. Pause
            // drawing for now.
            if self.safety < 100_000_000 {
                self.paused = true;
            }

            if !self.paused {
                self.step_robots(1);
            }
        }

        restore_terminal()?;
        Ok(())
    }

    pub fn step_robots(&mut self, n_steps: i32) {
        if n_steps == 0 || self.timestamp + n_steps < 0 {
            return;
        }

        self.timestamp += n_steps;

        let limits = self.counts.size().try_into().unwrap();
        for robot in &mut self.robots {
            self.counts[robot.pos] -= 1;
            robot.pos += robot.vel * n_steps;
            robot.pos.wrapping_clamp(&limits);
            self.counts[robot.pos] += 1;
        }
    }

    fn recalculate_stats(&mut self) {
        let (quad_counts, non_quad_count) = quadrant_counts(&self.robots, self.counts.width(), self.counts.height());

        self.quad_counts = quad_counts;
        self.non_quad_count = non_quad_count;
        self.safety = quad_counts.into_iter().fold(1, |a, c| a * c);

        if self.timestamp == 100 && self.safety100.is_none() {
            self.safety100 = Some(self.safety);
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        self.draw_robots(&mut stdout)?;
        self.draw_stats(&mut stdout)?;
        stdout.flush()
    }

    fn draw_robots(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        stdout
            .queue(SetBackgroundColor(Color::Black))?
            .queue(SetForegroundColor(Color::Green))?;

        // We need to iterate over the grid in chunks of 2 across and 8 down to properly generate braille characters.
        for (term_y, grid_y) in (0..self.counts.height()).step_by(C::Y_STEP).enumerate() {
            stdout.queue(MoveTo(0, term_y as u16))?;
            for grid_x in (0..self.counts.width()).step_by(C::X_STEP) {
                let c = C::make_char(&self.counts, grid_x, grid_y);
                stdout.queue(Print(c))?;
            }
        }

        stdout
            .queue(SetBackgroundColor(Color::Reset))?
            .queue(SetForegroundColor(Color::Reset))?;

        Ok(())
    }

    fn draw_stats(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        let x = (self.counts.width() / C::X_STEP + 3) as u16;

        macro_rules! info_line {
            ($y:expr, $($prints:expr),*) => {
                stdout
                .queue(MoveTo(x, $y))?
                $( .queue(Print($prints))? )*
                .queue(Clear(ClearType::UntilNewLine))?
            };
        }

        info_line!(0, "Timestamp: ", self.timestamp);
        info_line!(1, "Current safety factor: ", self.safety);

        if let Some(factor100) = &self.safety100 {
            info_line!(3, "Safety factor at t=100 seconds (part 1): ", factor100);
        } /* else {
        info_line!(3, "Safety factor at t=100 seconds (part 1): ", "(not reached yet)");
        } */

        let [q1, q2, q3, q4] = self.quad_counts;
        info_line!(5, "Robots in quadrant 1 (TL): ", q1);
        info_line!(6, "Robots in quadrant 2 (TR): ", q2);
        info_line!(7, "Robots in quadrant 3 (BL): ", q3);
        info_line!(8, "Robots in quadrant 4 (BR): ", q4);
        info_line!(10, "Robots not in any quadrant: ", self.non_quad_count);

        if self.paused {
            info_line!(12, "Simulation paused ⏸");
        } else {
            info_line!(12, ""); // clear the line
        }

        Ok(())
    }

    fn handle_events(&mut self, delay: Duration) -> io::Result<ControlFlow<()>> {
        // We only want to call `event::read` when we are ready to block; these two cases
        if self.paused || event::poll(delay)? {
            use KeyEventKind::Press;
            if let Event::Key(KeyEvent { code, kind: Press, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(ControlFlow::Break(())),
                    KeyCode::Char(' ') => self.paused = !self.paused,
                    KeyCode::Right if self.paused => self.step_robots(1),
                    KeyCode::Left if self.paused && self.timestamp > 0 => self.step_robots(-1),
                    _ => {},
                }
            }
        }

        Ok(ControlFlow::Continue(()))
    }
}

fn quadrant(pos: &Vec2, width: usize, height: usize) -> Option<u8> {
    let &Vec2 { x, y } = pos;
    let x_mid = (width / 2) as i32;
    let y_mid = (height / 2) as i32;

    if x == x_mid || y == y_mid {
        None
    } else {
        // bit 1 for horizontal, bit 2 for vertical
        // - 00 -> top left
        // - 01 -> top right
        // - 10 -> bottom left
        // - 11 -> bottom right
        let h = (x < x_mid) as u8;
        let v = (y < y_mid) as u8;
        Some(h | (v << 1))
    }
}

fn quadrant_counts<'a, I>(robots: I, width: usize, height: usize) -> ([u32; 4], u32)
where
    I: IntoIterator<Item = &'a Robot>,
{
    let mut quadrants = [0; 4];
    let mut outside = 0;

    for Robot { pos, .. } in robots {
        if let Some(i) = quadrant(pos, width, height) {
            quadrants[i as usize] += 1
        } else {
            outside += 1;
        }
    }

    (quadrants, outside)
}
