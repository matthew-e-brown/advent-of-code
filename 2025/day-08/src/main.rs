use std::str::FromStr;

fn main() {
    let input = aoc_utils::puzzle_input();
    let junctions = input.lines().map(|line| line.parse::<Junction>().unwrap()).collect::<Vec<_>>();

    println!("{junctions:#?}");
}

#[derive(Debug, Clone, Copy)]
struct Junction {
    x: u32,
    y: u32,
    z: u32,
}

impl Junction {
    /// Returns the squared Euclidean distance between this junction box and another.
    pub fn dist_sq(&self, other: &Junction) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        let dz = self.z.abs_diff(other.z) as u64;
        (dx * dx) + (dy * dy) + (dz * dz)
    }
}

impl FromStr for Junction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = s.split(',');
        let mut xyz = [0u32; 3];

        for i in 0..3 {
            xyz[i] = bits
                .next()
                .ok_or("junction box should have three fields")?
                .parse::<u32>()
                .map_err(|_| "junction box should be made of valid numbers")?;
        }

        if bits.count() != 0 {
            return Err("junction box should have three fields");
        }

        let [x, y, z] = xyz;
        Ok(Junction { x, y, z })
    }
}
