use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

use ahash::RandomState as AHashState;
use aoc_utils::clap;

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

            let i = u32::try_from(i).expect("only up to u32::MAX junctions boxes are supported");
            let j = j as u32; // j < i
            closest_pairs.push(Reverse(JunctionPair { dist, i, j }));
        }
    }

    // The next question becomes: now that I have this that sorted structure, how do I actually keep track of the
    // circuit layout between them?
    //
    // We need to keep track of: (1) which boxes are apart of which circuits, and (2) how large each circuit is. An
    // additional challenge here is that circuits can be merged together as we go. The most intuitive way to model this
    // is as a disconnected graph... Aha! Notice that our `JunctionPair` struct already looks like a pretty good
    // candidate for an `Edge` struct in an edge-list... If we just grab the top 1000 pairs, we will already have a
    // graph. Not only that, this lets us keep the `dist` field on them, just in case Part 2 ends up asking us about the
    // total size of each circuit (the wording of, "the Elves are concerned that they don't have enough extension cables
    // for all these circuits," makes me suspicious that that will be case...).
    //
    // The next challenge is that, according to the way the problem is phrased, edges between junctions which have
    // already been connected should not be made. We could run through a flood fill algorithm to determine the size of
    // each component, and that would work for part 1; but, if we *are* eventually going to need to determine the total
    // amount of wire used, we're going to have to eliminate those extra edges. So... let's just bite the bullet now and
    // convert our edge-list into a bunch of adjacency lists, and delete those unneeded edges now.

    let mut graph = CircuitGraph::new();
    let mut pairs_connected = 0usize;
    let mut largest_product = None;
    let mut final_pair = None;

    while let Some(Reverse(pair)) = closest_pairs.pop() {
        let p = pairs_connected + 1;

        if aoc_utils::verbosity() >= 2 {
            let JunctionPair { dist, i, j } = pair;
            let ji = junctions[i as usize];
            let jj = junctions[j as usize];
            println!("Closest pair #{p}: {ji:>17} and {jj:<17} (#{i:4} and #{j:4}), sq. dist = {dist}");
        }

        graph.add_edge(pair);
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
        } else if pairs_connected > closest_n && graph.num_nodes() == junctions.len() && graph.num_components() == 1 {
            // Now we're in the second phase, we want to stop once all nodes appear in the final graph
            if aoc_utils::verbosity() >= 1 {
                let JunctionPair { i, j, .. } = pair;
                let ji = junctions[i as usize];
                let jj = junctions[j as usize];
                println!("\nPair #{p} was last needed to create one circuit: {ji} and {jj} (#{i:4} and #{j:4})\n");
            }

            final_pair = Some(pair);
            break;
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
    // indices, but as u32 to shrink the struct size
    pub i: u32,
    pub j: u32,
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

#[derive(Debug, Clone)]
#[allow(unused)]
struct CircuitGraph {
    // so much heap allocation... this thing is gonna be enormous!
    /// The nodes of this graph.
    nodes: HashMap<u32, Node, AHashState>,

    /// The weight (distance) of each edge.
    ///
    /// This is not actually used for either part of this puzzle. But it was added during Part 1 in anticipation for
    /// Part 2, and I may as well keep it! It'll be good for future reference.
    edge_weights: HashMap<(u32, u32), u64, AHashState>,

    /// A "map" from component index/label to component size.
    ///
    /// A size of zero means that that component has been deleted, and that this index may be re-used for a new
    /// component.
    component_sizes: Vec<u64>,

    /// An *O*(1)-accessible component count. Avoids having to iterate through `component_sizes` and filter out the zero
    /// values.
    num_components: usize,
}

#[derive(Debug, Clone)]
struct Node {
    /// The index/label of the component this node is apart of.
    cmp: usize,
    /// This node's adjacency list.
    adj: Vec<u32>,
}

#[allow(unused)]
impl CircuitGraph {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            edge_weights: Default::default(),
            component_sizes: Default::default(),
            num_components: 0,
        }
    }

    /// Finds a free spot in [`self.component_sizes`] to create a new component with the given number of nodes, or
    /// creates a new component if there are no spots. The new component's index is returned.
    ///
    /// [`self.component_sizes`]: Self::component_sizes
    fn new_component(&mut self, count: u64) -> usize {
        self.num_components += 1;
        let len = self.component_sizes.len();
        if let Some(i) = (0..len).find(|&i| self.component_sizes[i] == 0) {
            self.component_sizes[i] = count;
            i
        } else {
            self.component_sizes.push(count);
            len
        }
    }

    /// Gets the number of unique nodes that are currently in this graph.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Adds a new node with no edges to this graph.
    pub fn add_node(&mut self, i: u32) {
        if !self.nodes.contains_key(&i) {
            // New nodes are never in an existing component
            let cmp_idx = self.new_component(1);
            self.nodes.insert(i, Node { cmp: cmp_idx, adj: vec![] });
        }
    }

    /// Adds up to two new nodes and their edge weight to this graph.
    pub fn add_edge(&mut self, pair: JunctionPair) {
        let JunctionPair { dist, i, j } = pair;

        // Are i and j already in the graph? Are they in the same component?
        let [node_i, node_j] = self.nodes.get_disjoint_mut([&i, &j]);

        // Convert `Option<&mut Node>` into `Ok<&mut Node>` for existing nodes, and `Err((this, other))` for not-found
        // nodes. This allows an individual match-arm down below to extract all the information it needs in one fell
        // swoop just by pattern matching, which allows us to merge two arms which would otherwise need to be mirrors
        // of one another. Said arms cannot be merged by extracting to a method, since we need Rust to see that our
        // disjoint mutable borrows are unused by the time we start fiddling with `self` again.
        let node_i = node_i.ok_or((i, j));
        let node_j = node_j.ok_or((j, i));

        match (node_i, node_j) {
            // Neither are in the graph: a new component is created for these two nodes and they are both added.
            (Err((i, _)), Err((j, _))) => {
                let cmp_idx = self.new_component(2);
                self.nodes.insert(i, Node { cmp: cmp_idx, adj: vec![j] });
                self.nodes.insert(j, Node { cmp: cmp_idx, adj: vec![i] });
                self.add_edge_weight(i, j, dist);

                if aoc_utils::verbosity() >= 3 {
                    println!("    {i} and {j} not connected yet.");
                    println!("    Added {i} and {j} to new component #{cmp_idx}.")
                }
            },
            // One is in the graph, and the other is not: the existing node has the other added to its adjacency list,
            // and a new node is created for the missing one, with the same component index. That component then gets
            // larger by one.
            (Ok(found_node), Err((not_found, found))) | (Err((not_found, found)), Ok(found_node)) => {
                let cmp_idx = found_node.cmp;
                found_node.adj.push(not_found);
                self.nodes.insert(not_found, Node { cmp: cmp_idx, adj: vec![found] });
                self.component_sizes[cmp_idx] += 1;
                self.add_edge_weight(i, j, dist);

                if aoc_utils::verbosity() >= 3 {
                    println!("    {found} was in graph in component #{cmp_idx}, but {not_found} as not.");
                    println!("    Added {not_found} to component #{cmp_idx}.");
                }
            },
            // Both are already in the same component: nothing happens.
            (Ok(node_i), Ok(node_j)) if node_i.cmp == node_j.cmp => {
                if aoc_utils::verbosity() >= 3 {
                    println!("    {i} and {j} already connected in component #{}.", node_j.cmp);
                }
            },
            // Both are in the graph, but not in the same component: the two components need to be merged. Neither one
            // gets bigger before the merge takes place, since both were already present.
            (Ok(node_i), Ok(node_j)) => {
                // (nodes are *not* in each other's adjacency lists: otherwise they would've been in the same component)
                node_i.adj.push(j);
                node_j.adj.push(i);
                let cmp_i = node_i.cmp;
                let cmp_j = node_j.cmp;

                if aoc_utils::verbosity() >= 3 {
                    println!("    {i} and {j} in graph but not in same component.");
                }

                self.merge_components((i, cmp_i), (j, cmp_j));
                self.add_edge_weight(i, j, dist);
            },
        }

        if aoc_utils::verbosity() >= 4 {
            println!(
                "    Graph has {} nodes and {} components. Components: {:?}",
                self.nodes.len(),
                self.num_components,
                self.component_sizes,
            );
        }
    }

    /// Adds an edge's weight to [`self.edge_weights`]. Ensures that `j < i` before inserting.
    ///
    /// [`self.edge_weights`]: Self::edge_weights
    fn add_edge_weight(&mut self, i: u32, j: u32, dist: u64) {
        // (technically, `j < i` should be guaranteed by how we initially constructed our JunctionPairs; but it can't
        // hurt to be thorough).
        let [min, max] = minmax(i, j);
        self.edge_weights.insert((max, min), dist);
    }

    #[allow(unused)]
    pub fn get_edge_weight(&self, i: u32, j: u32) -> Option<u64> {
        let [min, max] = minmax(i, j);
        self.edge_weights.get(&(max, min)).copied()
    }

    fn merge_components(&mut self, (i, cmp_i): (u32, usize), (j, cmp_j): (u32, usize)) {
        // Determine which component is larger; keep that one since it means fewer iterations when traversing to update
        // the nodes.
        let size_i = self.component_sizes[cmp_i];
        let size_j = self.component_sizes[cmp_j];
        assert!(size_i > 0 && size_j > 0, "components of size 0 should not be merged");

        // We need: (a) to know which component to merge into, (b) to know which to merge from, and (c) to have some
        // node within the merged-from component through which we can start the traversal, to tell all nodes of that
        // component to update their references.
        let (dst_cmp, src_cmp, src_node) = if size_i >= size_j { (cmp_i, cmp_j, j) } else { (cmp_j, cmp_i, i) };

        if aoc_utils::verbosity() >= 3 {
            println!(
                "    Merging component {src_cmp} (size {}) into {dst_cmp} (size {}).",
                self.component_sizes[src_cmp], self.component_sizes[dst_cmp],
            );
        }

        // Update the sizes
        self.component_sizes[dst_cmp] += self.component_sizes[src_cmp];
        self.component_sizes[src_cmp] = 0;

        // Traverse!
        let mut stack = vec![src_node];
        let mut visited = HashSet::<u32, AHashState>::default();
        while let Some(node_id) = stack.pop() {
            // `insert` returns true for new insertions
            if visited.insert(node_id) {
                let node = self
                    .nodes
                    .get_mut(&node_id)
                    .expect("nodes from adjacency list should be in graph");
                node.cmp = dst_cmp;
                stack.extend(&node.adj);
            }
        }

        self.num_components -= 1;
    }

    /// Gets an iterator of all the components' IDs and their sizes in this graph.
    pub fn component_sizes(&self) -> impl Iterator<Item = (usize, u64)> {
        self.component_sizes.iter().copied().enumerate().filter(|(_, size)| *size != 0)
    }

    /// Gets the number of components in this graph.
    pub fn num_components(&self) -> usize {
        self.num_components
    }
}

#[inline]
fn minmax<T: Ord>(a: T, b: T) -> [T; 2] {
    if a <= b { [a, b] } else { [b, a] }
}
