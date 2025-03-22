use std::str::FromStr;

const NUM_SECONDS: i32 = 100;
const MAP_W: i32 = 101;
const MAP_H: i32 = 103;

fn main() {
    let mut robots = aoc_utils::puzzle_input()
        .lines()
        .map(Robot::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut num_outside = 0u32;
    let mut quadrant_counts = [0u32; 4];
    for robot in &mut robots {
        // Simulate for 100 steps
        robot.pos.x = (robot.pos.x + robot.vel.x * NUM_SECONDS).rem_euclid(MAP_W);
        robot.pos.y = (robot.pos.y + robot.vel.y * NUM_SECONDS).rem_euclid(MAP_H);
        match quadrant(robot.pos) {
            Some(i) => quadrant_counts[i] += 1,
            None => num_outside += 1,
        }
    }

    if aoc_utils::verbosity() >= 1 {
        println!("Quadrant counts are: {quadrant_counts:?}.");
        println!("There are {num_outside} robots outside of (between) a quadrant.");
    }

    let safety_factor = quadrant_counts.iter().fold(1, |a, &c| a * c);
    println!("Total safety factor after {NUM_SECONDS} seconds (part 1): {safety_factor}");
}

fn quadrant(pos: Vec2) -> Option<usize> {
    const XL: i32 = MAP_W / 2 - 1;
    const XR: i32 = MAP_W / 2 + 1;
    const YU: i32 = MAP_H / 2 - 1;
    const YD: i32 = MAP_H / 2 + 1;
    match pos {
        Vec2 { x: 0..=XL, y: 0..=YU } => Some(0),
        Vec2 { x: 0..=XL, y: YD..MAP_H } => Some(1),
        Vec2 { x: XR..MAP_W, y: 0..=YU } => Some(2),
        Vec2 { x: XR..MAP_W, y: YD..MAP_H } => Some(3),
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

impl FromStr for Vec2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or("vec2 missing a comma")?;
        let x = x.parse().or(Err("failed to parse 'x'"))?;
        let y = y.parse().or(Err("failed to parse 'y'"))?;
        Ok(Vec2 { x, y })
    }
}

// impl Mul<i32> for Vec2 {
//     type Output = Vec2;

//     fn mul(self, rhs: i32) -> Self::Output {
//         Vec2 { x: self.x * rhs, y: self.y * rhs }
//     }
// }

// impl Mul<Vec2> for i32 {
//     type Output = Vec2;

//     fn mul(self, rhs: Vec2) -> Self::Output {
//         Vec2 { x: self * rhs.x, y: self * rhs.y }
//     }
// }

// impl AddAssign<Vec2> for Vec2 {
//     fn add_assign(&mut self, rhs: Vec2) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//     }
// }
