/// A set composed of the union of disjoint subsets of indices.
#[derive(Clone)]
pub struct DisjointSetUnion {
    map: Vec<isize>,
    num_sets: usize,
}

enum Entry {
    Root { size: usize },
    Child { next: usize },
}

impl DisjointSetUnion {
    /// Creates a new disjoint set union with the given number of nodes.
    pub fn new(n: usize) -> Self {
        assert!(n <= (isize::MAX as usize), "disjoint set union cannot hold more than isize::MAX sets");
        Self { map: vec![-1; n], num_sets: n }
    }

    /// Creates a new node in this disjoint set union.
    #[allow(unused)]
    pub fn add_node(&mut self) -> usize {
        self.num_sets += 1;
        self.map.push(-1);
        self.map.len() - 1
    }

    fn entry(&self, index: usize) -> Entry {
        match self.map[index] {
            x @ ..0 => Entry::Root { size: (-x) as usize },
            x @ 0.. => Entry::Child { next: x as usize },
        }
    }

    fn set_parent(&mut self, index: usize, parent: usize) {
        self.map[index] = parent as isize;
    }

    fn set_size(&mut self, index: usize, new_size: usize) {
        assert!(new_size <= (isize::MAX as usize), "disjoint subset cannot be larger than isize::MAX");
        self.map[index] = -(new_size as isize);
    }

    /// Returns the number of sets in this union.
    pub fn count(&self) -> usize {
        self.num_sets
    }

    /// Gets the size of the given node's subset.
    pub fn size(&self, node_index: usize) -> usize {
        match self.entry(node_index) {
            Entry::Root { size } => size,
            Entry::Child { next } => self.size(next),
        }
    }

    /// Gets the representative index for the node at the given index.
    #[allow(unused)]
    pub fn find_representative(&self, node_index: usize) -> usize {
        match self.entry(node_index) {
            Entry::Root { size: _ } => node_index,
            Entry::Child { next } => self.find_representative(next),
        }
    }

    /// Gets the representative index for the node at the given index, while also shortening the path to the root.
    fn find_representative_and_compress(&mut self, node_index: usize) -> usize {
        match self.entry(node_index) {
            Entry::Root { size: _ } => node_index,
            Entry::Child { next } => {
                let root = self.find_representative_and_compress(next);
                self.set_parent(node_index, root);
                root
            },
        }
    }

    /// Joins the sets containing `i` and `j` together. Returns `true` if the two subsets were merged, and `false` if
    /// the they were already in the same subset.
    pub fn join_subsets(&mut self, index_i: usize, index_j: usize) -> bool {
        let i = self.find_representative_and_compress(index_i);
        let j = self.find_representative_and_compress(index_j);
        if i == j {
            false
        } else {
            // Otherwise we wish to merge. Which set is larger?
            let si = self.size(i);
            let sj = self.size(j);

            // Keep the larger one as the root, since more things currently point to it.
            let (from, into) = if si < sj { (i, j) } else { (j, i) };
            self.set_parent(from, into);
            self.set_size(into, si + sj);

            self.num_sets -= 1;
            true
        }
    }

    /// Returns an iterator over the sizes of this disjoin set union's subsets.
    ///
    /// The first element in each tuple is the subset's index, and the second is its size.
    pub fn sizes(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.map.len()).filter_map(|i| match self.entry(i) {
            Entry::Root { size } => Some((i, size)),
            Entry::Child { next: _ } => None,
        })
    }
}
