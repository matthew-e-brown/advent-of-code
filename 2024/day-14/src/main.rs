mod charsets;
mod robot;

use std::error::Error;
use std::io::{self, StdoutLock, Write};
use std::ops::ControlFlow;
use std::process::ExitCode;
use std::time::Duration;

use aoc_utils::Grid;
use aoc_utils::clap::{self, Parser};
use crossterm::QueueableCommand;
use crossterm::cursor::{self, DisableBlinking, EnableBlinking, Hide as HideCursor, MoveTo, Show as ShowCursor};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
#[cfg(not(windows))]
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, DisableLineWrap, EnableLineWrap};

use self::charsets::Charset;
use self::robot::{Robot, Vec2};

pub const MAP_W: usize = 101;
pub const MAP_H: usize = 103;

#[derive(Debug, Parser)]
#[command(disable_help_flag = true)]
struct Args {
    /// How wide of a map to place the robots from the provided input file onto.
    ///
    /// This value is not provided as part of the main puzzle input, since it is supposed to be constant. This precludes
    /// using smaller grids for tests, however. This option provides a way to override the hardcoded default.
    #[arg(short, long, default_value_t = MAP_W)]
    width: usize,

    /// How tall of a map to place the robots from the provided input file onto.
    ///
    /// This value is not provided as part of the main puzzle input, since it is supposed to be constant. This precludes
    /// using smaller grids for tests, however. This option provides a way to override the hardcoded default.
    #[arg(short, long, default_value_t = MAP_H)]
    height: usize,

    /// Which character-set to use for printing the robot map.
    #[arg(short, long, value_enum, default_value_t = Charset::Braille)]
    charset: Charset,

    /// Print help (see a summary with '-?').
    #[arg(short = '?', long, action = clap::ArgAction::Help)]
    help: (),
}

fn main() -> ExitCode {
    let Args { width, height, charset, .. } = aoc_utils::parse_puzzle_args::<Args>();
    let robots = aoc_utils::puzzle_input().lines().map(|line| line.parse().unwrap());

    let app = App::new(width, height, charset, robots);

    match app.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        },
    }
}

struct App {
    // Simulation state:
    robots: Vec<Robot>,
    counts: Grid<u8>,
    safety: u32,
    quad_counts: [u32; 4],
    non_quad_count: u32,
    safety100: Option<u32>,
    timestamp: i32,
    paused: bool,
    // For rendering:
    charset: Charset,
    start_row: u16,
}

impl App {
    pub fn new(width: usize, height: usize, charset: Charset, robots: impl IntoIterator<Item = Robot>) -> Self {
        let mut counts = Grid::<u8>::empty(width, height);

        let robots = robots
            .into_iter()
            .inspect(|Robot { pos, .. }| {
                assert!((pos.x as usize) < width, "width is not large enough to accommodate all input robots");
                assert!((pos.y as usize) < height, "height is not large enough to accommodate all input robots");
                counts[*pos] += 1;
            })
            .collect();

        App {
            counts,
            robots,
            safety: 0,
            quad_counts: [0; 4],
            non_quad_count: 0,
            safety100: None,
            timestamp: 0,
            paused: false,
            charset,
            start_row: 0,
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.setup_terminal()?;
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

        self.restore_terminal()?;
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
        // ANSI 16 is Xterm's "Grey0": forces 0,0,0 rgb, as opposed to using the "black" from the user's terminal theme
        // (which is often just a dark grey).
        stdout
            .queue(SetBackgroundColor(Color::AnsiValue(16)))?
            .queue(SetForegroundColor(Color::Green))?;

        let x_step = self.charset.x_step();
        let y_step = self.charset.y_step();

        // We need to iterate over the grid in chunks of 2 across and 8 down to properly generate braille characters.
        for (term_y, grid_y) in (0..self.counts.height()).step_by(y_step).enumerate() {
            stdout.queue(MoveTo(0, self.start_row + term_y as u16))?;
            for grid_x in (0..self.counts.width()).step_by(x_step) {
                let c = self.charset.make_char(&self.counts, grid_x, grid_y);
                stdout.queue(Print(c))?;
            }
        }

        stdout
            .queue(SetBackgroundColor(Color::Reset))?
            .queue(SetForegroundColor(Color::Reset))?;

        Ok(())
    }

    fn draw_stats(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        let x = self.charset.grid_width_term(self.counts.width()) + 2;

        macro_rules! info_line {
            ($y:expr, $($prints:expr),*) => {
                stdout
                .queue(MoveTo(x, self.start_row + $y))?
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
        // We only want to call `event::read` when we are ready to block. When we're paused, we are always ready to
        // block; but when animating, the caller will pass in a Duration for how long they wish to delay the animation.
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

    fn setup_terminal(&mut self) -> Result<(), Box<dyn Error>> {
        // We want to print a blank line after our printed grid, just for some margins. So we need one more than the
        // grid's height.
        let rows_needed = self.charset.grid_height_term(self.counts.height()) + 1;
        let (_, term_rows) = terminal::size()?;
        (_, self.start_row) = cursor::position()?;

        if rows_needed > term_rows {
            return Err(format!(
                "The chosen character set is too large to fit on the current terminal screen! ({} rows > {} height)",
                rows_needed, term_rows,
            )
            .into());
        }

        // This will be the `y` position of the blank line after the printed grid.
        let bottom_row = self.start_row + rows_needed;

        // A scroll-up is needed if `bottom_row < term_rows`. If we have 70 total rows, and our bottom row would be 69
        // (the last row), then we're good. But if it would be 70, we need to move down by 1; ergo, `bottom - term_rows
        // + 1`, or `bottom - (term_rows - 1)`.
        if let Some(needed @ 1..) = bottom_row.checked_sub(term_rows - 1) {
            /* stdout.execute(ScrollUp(needed))?; */

            // Hack: Using the ANSI `ScrollUp` command seems to prevent the terminal from keeping the scrollback buffer
            // intact (at least, in Windows Terminal 2025-03-24). Doesn't seem to be 100% equivalent to pushing new
            // blank rows onto the bottom of the terminal.
            for _ in 0..needed {
                println!();
            }

            self.start_row -= needed;
        }

        let mut stdout = io::stdout().lock();

        // Non-Windows devices need an extra flag to be active to be able to read whether or not a KeyEvent is a
        // press/release/hold etc. That feature is always enabled on Windows, but trying to set it on Windows results in
        // a crash. So, it is gated with `cfg`.
        #[cfg(not(windows))]
        stdout.queue(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
        stdout
            .queue(DisableBlinking)?
            .queue(DisableLineWrap)?
            .queue(HideCursor)?
            .flush()?;
        Ok(())
    }

    fn restore_terminal(&mut self) -> io::Result<()> {
        // Based on where the cursor was when we set up the terminal, skip down by the right number of lines.
        let grid_height = self.charset.grid_height_term(self.counts.height());

        let mut stdout = io::stdout().lock();

        #[cfg(not(windows))]
        stdout.queue(PopKeyboardEnhancementFlags)?;
        stdout
            .queue(MoveTo(0, self.start_row + grid_height + 1))?
            .queue(EnableBlinking)?
            .queue(EnableLineWrap)?
            .queue(ShowCursor)?
            .flush()
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
