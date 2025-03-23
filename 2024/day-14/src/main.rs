use std::fmt::{Debug, Display};
use std::io::{self, Write};
use std::ops::{Add, AddAssign, Mul, SubAssign};
use std::process::ExitCode;
use std::str::FromStr;
use std::time::Duration;

use aoc_utils::Grid;
use aoc_utils::grid::GridIndex;
use crossterm::cursor::{DisableBlinking, EnableBlinking, Hide as HideCursor, MoveTo, Show as ShowCursor};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
#[cfg(not(windows))]
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};

const MAP_W: i32 = 101;
const MAP_H: i32 = 103;

const X_MID: i32 = MAP_W / 2;
const Y_MID: i32 = MAP_H / 2;


fn main() -> ExitCode {
    let input = aoc_utils::puzzle_input().lines().map(|line| Robot::from_str(line).unwrap());
    let map = Map::new(MAP_W as usize, MAP_H as usize, input);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableBlinking, HideCursor).unwrap();
    #[cfg(not(windows))]
    execute!(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)).unwrap();

    let result = run(map);

    execute!(stdout, LeaveAlternateScreen, EnableBlinking, ShowCursor).unwrap();

    #[cfg(not(windows))]
    execute!(PopKeyboardEnhancementFlags).unwrap();

    if let Err(err) = result {
        eprintln!("{err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run(mut map: Map) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut timestamp = 0u32;
    let mut safety_factor100 = None;

    let mut paused = false;

    let info_x = MAP_W as u16 + 2;

    queue!(stdout, Clear(ClearType::All))?;
    loop {
        // Display information from the previous iteration:
        map.display_robots(&mut stdout)?;

        let (quadrants, outside) = map.quadrant_counts();
        let safety_factor = Map::safety_factor(quadrants);
        if timestamp == 100 && safety_factor100.is_none() {
            safety_factor100 = Some(safety_factor);
        }

        queue!(stdout, MoveTo(info_x, 0), Print("Timestamp: "), Print(timestamp), Clear(ClearType::UntilNewLine))?;
        queue!(
            stdout,
            MoveTo(info_x, 1),
            Print("Safety factor: "),
            Print(safety_factor),
            Clear(ClearType::UntilNewLine)
        )?;
        if let Some(factor) = &safety_factor100 {
            queue!(
                stdout,
                MoveTo(info_x, 2),
                Print("Safety factor (100): "),
                Print(factor),
                Clear(ClearType::UntilNewLine)
            )?;
        }

        for y in 0..4 {
            let n = quadrants[y as usize];
            queue!(
                stdout,
                MoveTo(info_x, 4 + y),
                Print("Number in quadrant #"),
                Print(y),
                Print(": "),
                Print(n),
                Clear(ClearType::UntilNewLine)
            )?;
        }
        queue!(
            stdout,
            MoveTo(info_x, 9),
            Print("Number outside a quadrant: "),
            Print(outside),
            Clear(ClearType::UntilNewLine)
        )?;

        stdout.flush()?;

        let delay = if safety_factor < 200000000 {
            100
        } else {
            0
        };

        if event::poll(Duration::from_millis(delay))? {
            use Event::Key;
            use KeyEventKind::Press;
            if let Key(KeyEvent { kind: Press, code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Right if paused => {
                        timestamp += 1;
                        map.step_robots();
                    },
                    KeyCode::Left if paused && timestamp > 0 => {
                        timestamp -= 1;
                        map.step_robots_back();
                    },
                    _ => {},
                }
            }
        }

        // Now step the robots forwards:
        if !paused {
            timestamp += 1;
            map.step_robots();
        }
    }

    Ok(())
}


fn quadrant(pos: &Vec2) -> Option<usize> {
    const XM2: i32 = X_MID + 1;
    const YM2: i32 = Y_MID + 1;
    #[allow(non_contiguous_range_endpoints)] // We are intentionally excluding the midpoints
    match pos {
        Vec2 { x: 0..X_MID, y: 0..Y_MID } => Some(0),     // Top left
        Vec2 { x: XM2..MAP_W, y: 0..Y_MID } => Some(1),   // Top right
        Vec2 { x: 0..X_MID, y: YM2..MAP_H } => Some(2),   // Bottom left
        Vec2 { x: XM2..MAP_W, y: YM2..MAP_H } => Some(3), // Bottom right
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = s.split_whitespace();
        let p = bits.next().unwrap();
        let v = bits.next().ok_or("missing whitespace in robot input")?;
        let pi = p.find("p=").ok_or("robot pos missing 'p='")?;
        let vi = v.find("v=").ok_or("robot vel missing 'v='")?;
        let pxy = p.get(pi + 2..).ok_or("robot p= missing vector")?;
        let vxy = v.get(vi + 2..).ok_or("robot v= missing vector")?;
        Ok(Robot {
            pos: pxy.parse()?,
            vel: vxy.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl TryFrom<Vec2> for (usize, usize) {
    type Error = <usize as TryFrom<i32>>::Error;

    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x)?;
        let y = usize::try_from(value.y)?;
        Ok((x, y))
    }
}

impl TryFrom<(usize, usize)> for Vec2 {
    type Error = <i32 as TryFrom<usize>>::Error;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let x = i32::try_from(value.0)?;
        let y = i32::try_from(value.1)?;
        Ok(Vec2 { x, y })
    }
}

impl Vec2 {
    pub fn wrap(&mut self, limits: &Vec2) {
        self.x = self.x.rem_euclid(limits.x);
        self.y = self.y.rem_euclid(limits.y);
    }
}

impl FromStr for Vec2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or("vec2 missing a comma")?;
        let x = x.parse().or(Err("failed to parse 'x'"))?;
        let y = y.parse().or(Err("failed to parse 'y'"))?;
        Ok(Vec2 { x, y })
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(&(self.x, self.y), f)
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl GridIndex for Vec2 {
    fn x(&self) -> usize {
        usize::try_from(self.x).unwrap()
    }

    fn y(&self) -> usize {
        usize::try_from(self.y).unwrap()
    }

    fn from_xy(x: usize, y: usize) -> Self {
        (x, y).try_into().unwrap()
    }
}

struct Map {
    robots: Vec<Robot>,
    counts: Grid<u32>,
    // max_found: u32,
}

impl Map {
    /// Used for shading the colours
    const MAX_PER_CELL: u32 = 5;

    pub fn new(width: usize, height: usize, robots: impl IntoIterator<Item = Robot>) -> Self {
        assert!(width & 1 == 1, "width must be an odd number");
        assert!(height & 1 == 1, "height must be an odd number");

        let mut counts = Grid::empty(width, height);
        // let mut max_found = 0;
        let robots = robots
            .into_iter()
            .inspect(|robot| {
                counts[robot.pos] += 1;
                // max_found = u32::max(counts[robot.pos], max_found);
            })
            .collect();

        Map {
            robots,
            counts, /* max_found */
        }
    }

    // pub fn max_found(&self) -> u32 {
    //     self.max_found
    // }

    pub fn quadrant_counts(&self) -> ([u32; 4], u32) {
        let mut quadrants = [0; 4];
        let mut outside = 0;

        for Robot { pos, .. } in &self.robots {
            if let Some(i) = quadrant(pos) {
                quadrants[i] += 1
            } else {
                outside += 1;
            }
        }

        (quadrants, outside)
    }

    pub fn safety_factor(quadrants: [u32; 4]) -> u32 {
        quadrants.into_iter().fold(1, |a, c| a * c)
    }

    pub fn step_robots(&mut self) {
        let limits = self.counts.size().try_into().unwrap();
        for robot in &mut self.robots {
            self.counts[robot.pos] -= 1;
            robot.pos += robot.vel;
            robot.pos.wrap(&limits);
            self.counts[robot.pos] += 1;
            // self.max_found = self.max_found.max(self.counts[robot.pos]);
        }
    }

    pub fn step_robots_back(&mut self) {
        let limits = self.counts.size().try_into().unwrap();
        for robot in &mut self.robots {
            self.counts[robot.pos] -= 1;
            robot.pos -= robot.vel;
            robot.pos.wrap(&limits);
            self.counts[robot.pos] += 1;
            // self.max_found = self.max_found.max(self.counts[robot.pos]);
        }
    }

    pub fn display_robots(&self, out: &mut impl Write) -> io::Result<()> {
        for y in 0..self.counts.height() {
            queue!(out, MoveTo(0, y as u16), SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 }))?;
            let mut last_c = None;

            let is_h_mid = y == (Y_MID + 1) as usize;

            for x in 0..self.counts.width() {
                let n = self.counts[(x, y)];
                let c = u32::min(n * 255 / Self::MAX_PER_CELL, 255) as u8;

                let is_v_mid = x == (X_MID + 1) as usize;

                if last_c.is_none_or(|last| last != c) {
                    last_c = Some(c);
                    if c == 0 {
                        queue!(out, SetBackgroundColor(Color::Reset))?;
                    } else {
                        queue!(out, SetBackgroundColor(Color::Rgb { r: c, g: c, b: c }))?;
                    }
                }

                if n > 0 {
                    queue!(out, Print(n))?;
                } else {
                    let char = match (is_v_mid, is_h_mid) {
                        (false, false) => ' ',
                        (true, false) => '│', // U+2502 light vertical
                        (false, true) => '─', // U+2500 light horizontal
                        (true, true) => '┼',  // U+253C light both
                    };
                    queue!(out, Print(char))?;
                }
            }
        }

        queue!(out, ResetColor)?;
        Ok(())
    }
}
