use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::str::FromStr;

// [NOTE] This puzzle looks like it requires additional input: there are 1000 junctions, which means there are 1,000,000
// possible pairs of junctions; but we are only meant to connect the 1000 closest pairs. You'd think that means we're
// only meant to connect `n` pairs for `n` boxes, but the given example connects 10 pairs from 20 possible junctions.
// So, we'll accept this additional number on the command line. For now, we'll just hardcode it for printing.

fn main() {
    let input = aoc_utils::puzzle_input();
    let junctions = input.lines().map(|line| line.parse::<Junction>().unwrap()).collect::<Vec<_>>();

    // Actually... it doesn't really take that long to just compute all possible pairs of distances... 1000*1000 is only
    // 1,000,000. And, we don't actually need all 1000*1000; only one triangle of the matrix.
    //
    // What we'll do is store all these sizes in a sorted data structure, keyed by distance, with the value being the
    // (i, j) pair with that distance.
    //
    // The next question becomes: once I have that sorted structure, how do I actually keep track of the circuit layout
    // between them?

    let mut closest_pairs = BinaryHeap::new();
    for i in 0..junctions.len() {
        for j in 0..i {
            let a = &junctions[i];
            let b = &junctions[j];
            let dist = a.dist_sq(b);

            let i = u16::try_from(i).expect("only up to u16::MAX junctions boxes are supported");
            let j = j as u16; // j < i
            closest_pairs.push(Reverse(JunctionPair { dist, i, j }));
        }
    }

    if aoc_utils::verbosity() > 0 {
        for c in 0..100 {
            let Some(Reverse(JunctionPair { dist, i, j })) = closest_pairs.pop() else {
                break;
            };

            let i = i as usize;
            let j = j as usize;
            println!(
                "#{:>3} closest pair: ({i:4}, {j:4}): {:3} and {:3}, {dist:11} apart.",
                c + 1,
                junctions[i],
                junctions[j]
            );
        }
    }

    // Piped to stdout, this produces an 11.44 MiB text file containing a lookup table of all 1,000,000 distances in
    // just less than a second.
    /* let distances = aoc_utils::Grid::from_fn(junctions.len(), junctions.len(), |(i, j)| {
        let a = &junctions[i];
        let b = &junctions[j];
        a.dist_sq(b)
    });
    println!("{distances:>11?}");
    */
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

impl Display for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Junction { x, y, z } = self;
        if let Some(w) = f.width() {
            write!(f, "{x:>w$},{y:>w$},{z:>w$}")
        } else {
            write!(f, "{x},{y},{z}")
        }
    }
}

/// Metadata about a particular pair of [junction boxes][Junction].
///
/// An ordering is defined on this struct based solely on the [`dist`][Self::dist] field.
#[derive(Debug, Clone, Copy)]
struct JunctionPair {
    pub dist: u64,
    pub i: u16,
    pub j: u16,
}

impl PartialEq for JunctionPair {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for JunctionPair {}

impl Ord for JunctionPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for JunctionPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
