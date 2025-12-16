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
    graph.count_simple_paths("you", "out")
}

fn part2(graph: &Graph) -> Option<usize> {
    std::thread::scope(|scope| {
        // Instead of trying to enumerate all paths between 'svr' and 'out' and count how many contain 'fft' and 'dac',
        // we can be a bit more clever. Instead, we'll do them in segments: find all paths that go from 'svr'->'fft',
        // then join those with paths that go from 'fft'->'dac', then join those with paths that go 'dac'->'out. Then do
        // the same but for 'svr'->'dac', 'dac'->'fft', and 'fft'->'out'. This lets us do each part in its own thread
        // and lets us keep the simple path-counting algorithm!

        let svr_fft = scope.spawn(|| graph.count_simple_paths("svr", "fft"));
        let fft_dac = scope.spawn(|| graph.count_simple_paths("fft", "dac"));
        let dac_out = scope.spawn(|| graph.count_simple_paths("dac", "out"));

        let svr_dac = scope.spawn(|| graph.count_simple_paths("svr", "dac"));
        let dac_fft = scope.spawn(|| graph.count_simple_paths("dac", "fft"));
        let fft_out = scope.spawn(|| graph.count_simple_paths("fft", "out"));

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
    pub fn add_edge(&mut self, src: Label, dest: Label) -> bool {
        let adj_list = self.nodes.entry(src).or_default();
        match adj_list.binary_search(&dest) {
            Ok(_) => false,
            Err(i) => {
                adj_list.insert(i, dest);
                self.add_node(src);
                self.add_node(dest);
                true
            },
        }
    }

    /// Finds the number of simple paths between two vertices in this graph. Returns `None` if either the start or end
    /// vertex is not present in the graph.
    ///
    /// Sources:
    /// - <https://stackoverflow.com/a/21919879/10549827> for an explanation of the memoized implementation.
    /// - <https://stackoverflow.com/a/14089904/10549827> and
    ///   <https://www.baeldung.com/cs/simple-paths-between-two-vertices> for an overview of the original algorithm for
    ///   actually enumerating the paths, not just counting them.
    pub fn count_simple_paths(&self, src: Label, dest: Label) -> Option<usize> {
        if !self.nodes.contains_key(src) || !self.nodes.contains_key(dest) {
            return None;
        }

        // In a graph A->(B & C)->(...)->D is equal to the number of paths from B->D plus the number of paths from C->D.
        // However, those paths from B and C might be very similar, or even completely identical! We don't want to have
        // to do all that searching twice. So, we'll do the DFS scan from B->(...)->D as normal, but for every node N in
        // between, we'll store the number of N->(...)->D paths. Then, if/when C->(...)->D encounters N, it doesn't need
        // to travel down that whole leg again.

        let mut memo = HashMap::new(); // Doesn't need to store `dest` because that's the same for every iteration.
        let mut path = HashSet::new(); // Nodes in the path currently being explored; used for cycle detection.

        /// The recursive part of the algorithm.
        fn dfs_count_memo(
            graph: &Graph,
            src: Label,
            dest: Label,
            memo: &mut HashMap<Label, usize>,
            path: &mut HashSet<Label>,
        ) -> usize {
            if src == dest {
                // If we are trying to go from node A to node A, there is exactly one path: []. That is, as long as we
                // exclude all the possible A->A cycles. Which we do. Because we're counting simple paths.
                return 1;
            } else if path.contains(src) {
                // If we have already visited this node along the way, we have encountered a cycle. That wouldn't be a
                // simple path, so we do not count it.
                if aoc_utils::verbosity() >= 1 {
                    println!("Detected a cycle at node {src} on the way to {dest}.");
                }
                return 0;
            } else if let Some(&count) = memo.get(src) {
                // If we already know how many paths go from this node to the destination, just return that.
                count
            } else {
                // Otherwise, check how many are between each of our children and the destination and add those all up.
                path.insert(src);
                let mut count = 0;
                for neighbour in graph.nodes.get(src).as_deref().unwrap() {
                    count += dfs_count_memo(graph, neighbour, dest, memo, path);
                }
                memo.insert(src, count);
                path.remove(src);
                count
            }
        }

        let count = dfs_count_memo(self, src, dest, &mut memo, &mut path);

        if aoc_utils::verbosity() >= 2 {
            println!("Paths between {src} and {dest}: {count}");
        }

        Some(count)
    }
}
