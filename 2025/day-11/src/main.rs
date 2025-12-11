use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

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

    let paths_you_out = graph.all_simple_paths("you", "out");
    let paths_svr_out = graph.all_simple_paths("svr", "out");

    print!("Number of paths from 'you' to 'out' (part 1): ");
    if let Some(paths) = paths_you_out {
        println!("{}", paths.len());
    } else {
        println!("None");
    }

    print!("Number of paths from 'svr' to 'out' which visit 'fft' and 'dac' (part 2): ");
    if let Some(paths) = paths_svr_out {
        let mut num_fft_dac = 0usize;

        'paths: for path in paths {
            let mut fft = false;
            let mut dac = false;
            for label in path {
                if label == "fft" {
                    fft = true;
                } else if label == "dac" {
                    dac = true;
                }

                if fft && dac {
                    num_fft_dac += 1;
                    continue 'paths;
                }
            }
        }

        println!("{num_fft_dac}");
    } else {
        println!("None");
    }
}

/// A label for a vertex in a graph.
type Label = &'static str;

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

    /// Finds all simple paths between two vertices in this graph.
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
                    dfs(graph, neighbour, dst, visited, all_paths, curr_path);
                }

                // Now we're done all paths that involve this node, we can "un-visit" it.
                visited.remove(src);
                curr_path.pop();
            }
        }

        let mut visited = HashSet::new();
        let mut all_paths = Vec::new();
        let mut curr_path = Vec::new();

        dfs(self, source, destination, &mut visited, &mut all_paths, &mut curr_path);

        Some(all_paths)
    }
}
