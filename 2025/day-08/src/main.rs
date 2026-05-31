mod set;

use std::cmp::Reverse;
use std::fmt::Display;
use std::str::FromStr;

use aoc_utils::clap;

use crate::set::DisjointSetUnion;

// This puzzle requires additional input not present in the input text file.
//
// There are 1000 junctions, which means there are 1,000,000 possible pairs of junctions (or, 500,500 if you exclude
// duplicates and self-connections). The puzzle asks us to find the 1000 closest pairs; you might expect that the count
// of 1000 pairs would be derived from the 1000 junctions in the input. However, the example problem, with 20 junctions,
// only finds the 10 closest pairs! That means that, if we want this program to solve both the full input and the
// example, we'll have to be able to control how many pairs we connect.
//
// I'll accept this extra number on the command line for the sake of correctness, but fallback to defaults that match
// the full puzzle.
#[derive(Debug, clap::Parser)]
#[command(disable_help_flag = true)]
struct Args {
    /// For part 1, connect the N closest pairs of junction boxes and into circuits.
    ///
    /// For small inputs (< 50 junction boxes), the default is 10 (to match the example input). For larger inputs, the
    /// default is to connect the same number of pairs as there are junction boxes in the input.
    #[arg(short = 'n', value_name = "N")]
    closest_n: Option<usize>,

    /// After connecting the N closest junction pairs, we multiply the sizes of the M largest circuits together to get
    /// our final output for Part 1.
    ///
    /// The default is 3.
    #[arg(short = 'm', value_name = "M")]
    largest_m: Option<usize>,

    #[arg(short = '?', long, action = clap::ArgAction::Help)]
    help: (),
}

fn main() {
    // - We start by parsing all the junctions into a single list right from the get-go. We'll use their indices to
    //   refer to them throughout the rest of the solution.
    // - We immediately compute all possible pairs of indices; there aren't really that many. This list excludes
    //   self-references and duplicates.
    // - We use a disjoint set union to keep track of which junctions are connected to one another.
    let junctions = aoc_utils::puzzle_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<JunctionBox>>();
    let pairs = compute_sorted_pairs(&junctions[..]);
    let mut circuits = DisjointSetUnion::new(junctions.len());

    // Puzzle parameters depend slightly based on the size of the input:
    let Args { closest_n, largest_m, .. } = aoc_utils::parse_puzzle_args::<Args>();
    let closest_n = closest_n.unwrap_or(if junctions.len() < 50 { 10 } else { junctions.len() });
    let largest_m = largest_m.unwrap_or(3);

    if closest_n > pairs.len() {
        panic!(
            "Invalid puzzle parameters: attempted to find closest N={} pairs, but only {} pairs exist",
            closest_n,
            pairs.len(),
        );
    }

    let mut pairs_connected = 0;
    let mut largest_product = None;
    let mut final_x_product = None;

    for (i, j) in pairs {
        // Used for verbose printing:
        let p = pairs_connected + 1;

        if aoc_utils::verbosity() >= 2 {
            let ji = &junctions[i];
            let jj = &junctions[j];
            let ci = circuits.find_rep(i);
            let cj = circuits.find_rep(j);
            let dist = ji.dist_sq(&jj);
            println!(
                "Closest pair #{p}: {ji:>17} (#{i:4}, circuit #{ci:4}) and {jj:<17} (#{j:4}, circuit #{cj:4}), sq. dist = {dist}"
            );
        }

        circuits.join_subsets(i, j);
        pairs_connected += 1;

        // Part 1: we hit 1000! Now we can inspect the disjoint set and see how large all the circuits are.
        if pairs_connected == closest_n {
            let mut circuit_sizes = circuits.sizes().collect::<Vec<_>>();
            circuit_sizes.sort_unstable_by_key(|&(_, size)| Reverse(size));

            if largest_m > circuit_sizes.len() {
                panic!(
                    "Invalid puzzle parameters: attempted to find largest M={} circuits, but only {} circuits exist",
                    largest_m,
                    circuit_sizes.len(),
                );
            }

            if aoc_utils::verbosity() >= 1 {
                println!("\nCircuits after joining the closest {closest_n} pairs (index, size):\n{circuit_sizes:?}");
                if aoc_utils::verbosity() >= 2 {
                    println!();
                }
            }

            largest_product = Some(
                circuit_sizes
                    .into_iter()
                    .take(largest_m)
                    .map(|(_, size)| size)
                    .reduce(|a, c| a * c)
                    .unwrap(),
            );
        }

        // Part 2: keep going until we hit one single circuit. That is then the last pair we care about.
        if circuits.count() == 1 {
            let ji = &junctions[i];
            let jj = &junctions[j];

            if aoc_utils::verbosity() >= 1 {
                println!("\nPair #{p} was last needed to create one circuit: {ji} and {jj} (#{i:4} and #{j:4})\n");
            }

            final_x_product = Some((ji.x as u64) * (jj.x as u64));
            break;
        }
    }

    // Both of these are safe to unwrap. For part 1, we already checked that `closest_n` was low enough to be reached.
    // For part 2, we are guaranteed to always end up with a single circuit: in the very worst case, we'll end up going
    // through all possible pairs, but we will always end by connecting the last of them together.
    let largest_product = largest_product.unwrap();
    let final_x_product = final_x_product.unwrap();
    println!("Product of the largest {largest_m} circuits' sizes (part 1): {largest_product}");
    println!("Product of final pair of junction boxes' X-coordinates (part 2): {final_x_product}");
}

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    /// Returns the squared Euclidean distance between this junction box and another.
    pub fn dist_sq(&self, other: &JunctionBox) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        let dz = self.z.abs_diff(other.z) as u64;
        (dx * dx) + (dy * dy) + (dz * dz)
    }
}

impl FromStr for JunctionBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = s.split(',');
        let mut xyz = [0u32; 3];

        for i in 0..3 {
            xyz[i] = bits
                .next()
                .ok_or("junction box has less than 3 fields")?
                .parse::<u32>()
                .map_err(|_| "junction box contains invalid numbers")?;
        }

        if bits.count() != 0 {
            return Err("junction box has more than 3 fields");
        }

        let [x, y, z] = xyz;
        Ok(Self { x, y, z })
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Alignment;

        let JunctionBox { x, y, z } = self;
        let str = format!("{x},{y},{z}");

        match (f.width(), f.align()) {
            (Some(w), Some(Alignment::Left)) => write!(f, "{str:<w$}"),
            (Some(w), Some(Alignment::Right)) => write!(f, "{str:>w$}"),
            (Some(w), Some(Alignment::Center)) => write!(f, "{str:^w$}"),
            (Some(w), None) => write!(f, "{str:w$}"),
            (None, _) => write!(f, "{str}"),
        }
    }
}

/// Computes a list of all possible `(i, j)` pairs without any self-references or repeats.
fn compute_sorted_pairs(junctions: &[JunctionBox]) -> Vec<(usize, usize)> {
    // Summation identity: `sum_(i=0)^(n-1)(sum_(j=0)^(i-1)(1)) = (1/2)(n-1)n` (WolframAlpha)
    // (imagine a bottom-left triangular adjacency matrix).
    let num_pairs = (junctions.len() - 1) * junctions.len() / 2;
    let mut pairs = Vec::with_capacity(num_pairs);
    for i in 0..junctions.len() {
        for j in 0..i {
            pairs.push((i, j));
        }
    }

    pairs.sort_unstable_by_key(|&(i, j)| junctions[i].dist_sq(&junctions[j]));
    pairs
}
