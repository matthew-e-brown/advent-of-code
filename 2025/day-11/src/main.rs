use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

/// A label for a vertex in a graph.
type Label = &'static str;

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut graph = Graph::new();
    for line in input.lines() {
        let (source, rest) = line.split_once(':').expect("puzzle input lines should contain a colon");
        graph.add_node(source);
        for dest in rest.split_whitespace() {
            graph.add_edge(source, dest);
        }
    }

    let num_paths_you_out = part1(&graph);
    let num_paths_svr_out = part2(&graph);

    match num_paths_you_out {
        Some(n) => println!("Number of paths from 'you' to 'out' (part 1): {n}"),
        None => println!("Number of paths from 'you' to 'out' (part 1): None"),
    }

    match num_paths_svr_out {
        Some(n) => println!("Number of paths from 'svr' to 'out' which visit 'fft' and 'dac' (part 2): {n}"),
        None => println!("Number of paths from 'svr' to 'out' which visit 'fft' and 'dac' (part 2): None"),
    }
}

fn part1(graph: &Graph) -> Option<usize> {
    graph.all_simple_paths("you", "out").map(|v| v.len())
}

fn part2(graph: &Graph) -> Option<usize> {
    std::thread::scope(|scope| {
        // Simply finding all paths between 'svr' and 'out' takes *way* too long with the naïve approach. So we need to
        // be more clever: instead, we'll do them in segments.
        //
        // Find all paths that go from 'svr'->'fft', then join those with paths that go from 'fft'->'dac', then join
        // those with paths that go 'dac'->'out. Then do the same but for 'svr'->'dac', 'dac'->'fft', and 'fft'->'out'.

        let svr_fft = scope.spawn(|| graph.all_simple_paths("svr", "fft").map(|v| v.len()));
        let fft_dac = scope.spawn(|| graph.all_simple_paths("fft", "dac").map(|v| v.len()));
        let dac_out = scope.spawn(|| graph.all_simple_paths("dac", "out").map(|v| v.len()));

        let svr_dac = scope.spawn(|| graph.all_simple_paths("svr", "dac").map(|v| v.len()));
        let dac_fft = scope.spawn(|| graph.all_simple_paths("dac", "fft").map(|v| v.len()));
        let fft_out = scope.spawn(|| graph.all_simple_paths("fft", "out").map(|v| v.len()));

        let svr_fft = svr_fft.join().expect("thread poisoned")?;
        let fft_dac = fft_dac.join().expect("thread poisoned")?;
        let dac_out = dac_out.join().expect("thread poisoned")?;

        let svr_dac = svr_dac.join().expect("thread poisoned")?;
        let dac_fft = dac_fft.join().expect("thread poisoned")?;
        let fft_out = fft_out.join().expect("thread poisoned")?;

        // If there are 2 paths from A->B, and 2 paths from B->C, then there are 4 total paths from A->C: `A↗B↗C`,
        // `A↘B↗C`, `A↗B↘C`, `A↘B↘C` (where '↗' and '↘' are imaginary "up" and "down" paths).
        //
        // Let's just make the gung-ho assumption that that generalizes super well, and that the number of total
        // possible paths can be found by multiplying the bits together.

        let svr_fft_dac_out = svr_fft * fft_dac * dac_out;
        let svr_dac_fft_out = svr_dac * dac_fft * fft_out;
        Some(svr_fft_dac_out + svr_dac_fft_out)
    })
}

// How many times am I going to implement a graph this year?
#[derive(Debug)]
struct Graph {
    nodes: HashMap<Label, Vec<Label>>,
}

impl Graph {
    /// Creates a new graph.
    pub fn new() -> Self {
        Self { nodes: Default::default() }
    }

    /// Adds a node to this graph.
    ///
    /// Returns `true` if this is a new node.
    pub fn add_node(&mut self, label: Label) -> bool {
        match self.nodes.entry(label) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                entry.insert(Vec::new());
                true
            },
        }
    }

    /// Adds a single edge to this graph.
    ///
    /// `true` is returned if this edge was already in the graph.
    pub fn add_edge(&mut self, source: Label, destination: Label) -> bool {
        let adj_list = self.nodes.entry(source).or_default();
        match adj_list.binary_search(&destination) {
            Ok(_) => false,
            Err(i) => {
                adj_list.insert(i, destination);
                self.add_node(source);
                self.add_node(destination);
                true
            },
        }
    }

    /// Finds all simple paths between two vertices in this graph. Returns `None` if either the start or end vertex is
    /// not present in the graph.
    ///
    /// See:
    /// - <https://stackoverflow.com/a/14089904/10549827>
    /// - <https://www.baeldung.com/cs/simple-paths-between-two-vertices>
    pub fn all_simple_paths(&self, source: Label, destination: Label) -> Option<Vec<Box<[Label]>>> {
        if !self.nodes.contains_key(source) || !self.nodes.contains_key(destination) {
            return None;
        }

        /// The recursive part of the algorithm.
        fn dfs(
            graph: &Graph,
            src: Label,
            dst: Label,
            visited: &mut HashSet<Label>,
            all_paths: &mut Vec<Box<[Label]>>,
            curr_path: &mut Vec<Label>,
        ) {
            // `visited` contains all the nodes we've seen so far on this current path; if we've already seen this node
            // before, we have a cycle.
            if visited.contains(src) {
                if aoc_utils::verbosity() >= 1 {
                    println!("Cycle detected at vertex {src}.");
                }
                return;
            }

            curr_path.push(src);

            // If we've made it to the end, we're done; we can push this to our list.
            if src == dst {
                if aoc_utils::verbosity() >= 1 {
                    println!("Found path: {curr_path:?}");
                }

                // Need to clone the current path, since we're now about to step back up into the previous `dfs` call,
                // which may find another path (which starts the same way as this one did, but may end differently).
                all_paths.push(curr_path.clone().into_boxed_slice());
                curr_path.pop(); // Un-add this node from the current path
            } else {
                // Otherwise, we want to keep going:
                visited.insert(src);
                for neighbour in graph.nodes.get(src).as_deref().unwrap() {
                    if aoc_utils::verbosity() >= 2 {
                        println!("\tStepping from {src} to {neighbour}...");
                    }

                    dfs(graph, neighbour, dst, visited, all_paths, curr_path);
                }

                // Now we're done all paths that involve this node, we can "un-visit" it.
                visited.remove(src);
                curr_path.pop();
            }
        }

        if aoc_utils::verbosity() >= 1 {
            println!("Finding all simple paths between {source} and {destination}...");
        }

        let mut visited = HashSet::new();
        let mut all_paths = Vec::new();
        let mut curr_path = Vec::new();

        dfs(self, source, destination, &mut visited, &mut all_paths, &mut curr_path);

        Some(all_paths)
    }
}
