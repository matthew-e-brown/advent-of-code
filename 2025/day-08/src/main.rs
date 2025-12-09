mod graph;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::str::FromStr;

use aoc_utils::clap;

use self::graph::CircuitGraph;

// This puzzle seems to require additional input. There are 1000 junctions, which means there are 1,000,000 possible
// pairs of junctions (500,500 if you exclude duplicates and self-connections). The puzzle asks us to find the 1000
// closest pairs, which you'd expect is derived from the number of junctions in the input; but the example problem with
// 10 junctions only finds the 10 closest pairs! So I'll accept this extra number on the command line for the sake of
// correctness, but fallback to sensible defaults.
#[derive(Debug, clap::Parser)]
#[command(disable_help_flag = true)]
struct Args {
    /// For part 1, find the `n` closest pairs of junction boxes and connect them into circuits.
    ///
    /// For small inputs (< 50 junction boxes), the default is 10 (to match the example input). For larger inputs, the
    /// default is to connect the same number of pairs as there are junction boxes in the input.
    #[arg(short = 'n')]
    closest_n: Option<usize>,

    #[arg(short = '?', long, action = clap::ArgAction::Help)]
    help: (),
}

/// For part 1, after connecting the *n* closest pairs, we want to find the *m* largest circuits.
const LARGEST_M: usize = 3;

/// The type used to refer to a [Junction] by index.
///
/// [`u32`] is used instead of [`usize`] to keep struct sizes small.
type Idx = u32;

fn main() {
    let input = aoc_utils::puzzle_input();
    let junctions = input.lines().map(|line| line.parse::<Junction>().unwrap()).collect::<Vec<_>>();

    let Args { closest_n, .. } = aoc_utils::parse_puzzle_args::<Args>();
    let closest_n = closest_n.unwrap_or(if junctions.len() < 50 { 10 } else { junctions.len() });

    // This is the sort of puzzle that just screams, "there must be some key insight that will transform this problem
    // into some existing problem with a known, elegant algorithm!" But... it actually doesn't take that long to just
    // compute all the possible pairs of distances; there are only ~500K.
    //
    // What we'll do is store all these sizes in a sorted data structure, keyed by distance, with the value being the
    // (i, j) pair with that distance. (Instead of literally "keying" by distance, we'll just sort them by distance in a
    // binary heap; should have less overhead than a BTree with all its nodes.)

    let mut closest_pairs = BinaryHeap::new();
    for i in 0..junctions.len() {
        for j in 0..i {
            let a = &junctions[i];
            let b = &junctions[j];
            let dist = a.dist_sq(b);

            let i = Idx::try_from(i).expect("overflow: too many junction boxes");
            let j = j as Idx; // j < i
            closest_pairs.push(Reverse(JunctionPair { dist, i, j }));
        }
    }

    // The next question becomes: now that I have this that sorted structure, how do I actually keep track of the
    // circuit layout between them?
    //
    // We need to keep track of: (1) which boxes are apart of which circuits, and (2) how large each circuit is. An
    // additional challenge here is that circuits can be merged together as we go. The most intuitive way to model this
    // is as a graph with multiple components... Aha! Notice that our `JunctionPair` struct already looks like a pretty
    // good candidate for an `Edge` struct in an edge-list... If we just grab the top 1000 pairs, we essentially already
    // have a graph.
    //
    // That said, it's not quite as simple as just using the first 1000 pairs as an edge-list. When connecting up our
    // graph, we have to ensure that we don't add any edges between pairs already in the same component. I thought long
    // and hard other, simpler-to-implement ways to handle that, but... in the end, I decided it was probably for the to
    // just actually build a disconnected graph data structure. Much more elegant than having a bunch of random hashmaps
    // and vectors sitting around as local variables.

    let mut graph = CircuitGraph::new();
    let mut pairs_connected = 0usize;
    let mut largest_product = None;
    let mut final_pair = None;

    while let Some(Reverse(pair)) = closest_pairs.pop() {
        let p = pairs_connected + 1;
        let JunctionPair { i, j, dist } = pair;

        if aoc_utils::verbosity() >= 2 {
            let ji = junctions[i as usize];
            let jj = junctions[j as usize];
            println!("Closest pair #{p}: {ji:>17} and {jj:<17} (#{i:4} and #{j:4}), sq. dist = {dist}");
        }

        graph.add_edge(i, j, dist);
        pairs_connected += 1;

        if pairs_connected == closest_n {
            // Now that we've done the first `n`, find the largest component sizes for part 1. Once we've written that
            // down, keep going for part 2!
            let mut largest_circuits = graph.component_sizes().collect::<Vec<_>>();

            if aoc_utils::verbosity() >= 1 {
                println!("\nCircuits after joining the closest {closest_n} pairs (ID, size):\n{largest_circuits:?}");
            }
            if aoc_utils::verbosity() >= 2 {
                println!();
            }

            largest_circuits.sort_by_key(|&(_, size)| Reverse(size));
            largest_product = Some(
                largest_circuits
                    .into_iter()
                    .take(LARGEST_M)
                    .map(|(_, size)| size)
                    .reduce(|a, c| a * c)
                    .unwrap_or(0),
            );
        }

        if graph.num_nodes() == junctions.len() && graph.num_components() == 1 {
            // Now we're in the second phase, we want to stop once all nodes appear in the final graph
            if aoc_utils::verbosity() >= 1 {
                let ji = junctions[i as usize];
                let jj = junctions[j as usize];
                println!("\nPair #{p} was last needed to create one circuit: {ji} and {jj} (#{i:4} and #{j:4})\n");
            }

            final_pair = Some(pair);
            break; // Everything is now connected, we can stop now.
        }
    }

    let Some(largest_product) = largest_product else {
        panic!("Invalid puzzle input: not enough junction boxes provided to create {closest_n} pairs.");
    };

    // This one is safe to unwrap, because there will *always* be enough to get things down to one component: in the
    // worst case, you'd connect every single pair. The challenge of part 2 is finding out when it happens early.
    let final_pair = final_pair.unwrap();
    let final_i = &junctions[final_pair.i as usize];
    let final_j = &junctions[final_pair.j as usize];
    let final_x_product = (final_i.x as u64) * (final_j.x as u64);

    println!("Product of the largest {LARGEST_M} circuits' sizes (part 1): {largest_product}");
    println!("Product of final pair of junction boxes' X-coordinates (part 2): {final_x_product}");
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
        use std::fmt::Alignment;

        let Junction { x, y, z } = self;
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

/// Metadata about a particular pair of [junction boxes][Junction].
///
/// An ordering is defined on this struct based solely on the [`dist`][Self::dist] field.
#[derive(Debug, Clone, Copy)]
struct JunctionPair {
    pub dist: u64,
    pub i: Idx,
    pub j: Idx,
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
