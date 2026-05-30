#![allow(dead_code)]
// ↑↑ I did a bit more work than I needed to for this graph implementation. Most notably, I didn't end up needing to
// track edge weights. But I added most of those extra bits during Part 1 in anticipation for Part 2, and they weren't
// really hurting anything. So I figured I'd just leave them here. Why not? They'll be good future reference next time I
// need a graph! :)

use std::collections::{HashMap, HashSet};

use ahash::RandomState as AHashState;

use super::Idx;

/// A weighted, undirected graph that keeps track of connected components.
#[derive(Debug, Clone)]
pub struct CircuitGraph {
    // so much heap allocation... this thing is gonna be enormous!
    /// The nodes of this graph.
    nodes: HashMap<Idx, Node, AHashState>,

    /// The weight (distance) of each edge.
    edge_weights: HashMap<(Idx, Idx), u64, AHashState>,

    /// A "map" from component index/label to component size.
    ///
    /// A size of zero means that that component has been deleted, and that this index may be re-used for a new
    /// component.
    component_sizes: Vec<usize>,

    /// An *O*(1)-accessible component count. Avoids having to iterate through `component_sizes` and filter out the zero
    /// values.
    num_components: usize,
}

#[derive(Debug, Clone)]
struct Node {
    /// The index/label of the component this node is apart of.
    cmp: usize,
    /// This node's adjacency list.
    adj: Vec<Idx>,
}

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
    fn new_component(&mut self, count: usize) -> usize {
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
    pub fn add_node(&mut self, i: Idx) {
        if !self.nodes.contains_key(&i) {
            // New nodes are never in an existing component
            let cmp_idx = self.new_component(1);
            self.nodes.insert(i, Node { cmp: cmp_idx, adj: vec![] });
        }
    }

    /// Adds up to two new nodes and their edge weight to this graph.
    pub fn add_edge(&mut self, i: Idx, j: Idx, dist: u64) {
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
    fn add_edge_weight(&mut self, i: Idx, j: Idx, dist: u64) {
        // (technically, `j < i` should be guaranteed by how we initially constructed our JunctionPairs; but it can't
        // hurt to be thorough).
        let [min, max] = minmax(i, j);
        self.edge_weights.insert((max, min), dist);
    }

    /// Gets the weight of an edge in this graph, if it exists.
    pub fn get_edge_weight(&self, i: Idx, j: Idx) -> Option<u64> {
        let [min, max] = minmax(i, j);
        self.edge_weights.get(&(max, min)).copied()
    }

    fn merge_components(&mut self, (i, cmp_i): (Idx, usize), (j, cmp_j): (Idx, usize)) {
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
        let mut visited = HashSet::<Idx, AHashState>::default();
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
    ///
    /// The first item in each tuple is the component's index, and the second is its size.
    pub fn component_sizes(&self) -> impl Iterator<Item = (usize, usize)> {
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
