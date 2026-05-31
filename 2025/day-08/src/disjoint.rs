/// A disjoint set union.
///
/// This structure keeps track of some [`len`][Self::len] number of **elements** which are joined together into
/// **subsets**. Each element is represented by an index. The subset that an element belongs to is represented by
/// the index of the subset's **root** element.
#[derive(Clone)]
pub struct DisjointSet {
    map: Vec<isize>,
    num_sets: usize,
}

enum Entry {
    Root { size: usize },
    Child { next: usize },
}

impl DisjointSet {
    /// Creates a new disjoint set union without any subsets.
    #[expect(unused)]
    pub fn new() -> Self {
        Self { map: Vec::new(), num_sets: 0 }
    }

    /// Returns the total number of elements across all sets in this union.
    #[expect(unused)]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns the number of sets in this union.
    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    /// Creates a new disjoint set union with the given number of nodes.
    pub fn with_len(n: usize) -> Self {
        assert!(n <= (isize::MAX as usize), "disjoint set union cannot hold more than isize::MAX sets");
        Self { map: vec![-1; n], num_sets: n }
    }

    /// Creates a new subset containing a single element.
    ///
    /// The index of the new element (the root of the new subset) is returned.
    #[expect(unused)]
    pub fn push_single(&mut self) -> usize {
        let index = self.map.len();
        self.map.push(-1);
        self.num_sets += 1;
        index
    }

    fn entry(&self, index: usize) -> Entry {
        match self.map[index] {
            x @ ..0 => Entry::Root { size: (-x) as usize },
            x @ 0.. => Entry::Child { next: x as usize },
        }
    }

    #[inline]
    fn set_parent(&mut self, index: usize, parent: usize) {
        self.map[index] = parent as isize;
    }

    #[inline]
    fn set_size(&mut self, index: usize, new_size: usize) {
        assert!(new_size <= (isize::MAX as usize), "disjoint subset cannot be larger than isize::MAX");
        self.map[index] = -(new_size as isize);
    }

    /// Gets the size of the subset `index` is a part of.
    pub fn size_of(&self, index: usize) -> usize {
        match self.entry(index) {
            Entry::Root { size } => size,
            Entry::Child { next } => self.size_of(next),
        }
    }

    /// Checks if the indices `i` and `j` belong to the same subset.
    #[expect(unused)]
    pub fn is_joined(&self, i: usize, j: usize) -> bool {
        self.find_root(i) == self.find_root(j)
    }

    /// Finds the root index of the subset containing `index`.
    pub fn find_root(&self, index: usize) -> usize {
        match self.entry(index) {
            Entry::Root { size: _ } => index,
            Entry::Child { next } => self.find_root(next),
        }
    }

    /// Finds the root index of the subset containing `index`, while performing path compression on the way.
    pub fn find_root_and_compress(&mut self, index: usize) -> usize {
        match self.entry(index) {
            Entry::Root { size: _ } => index,
            Entry::Child { next } => {
                let root = self.find_root_and_compress(next);
                self.set_parent(index, root); // Path compression
                root
            },
        }
    }

    /// Joins the subsets containing `i` and `j` together.
    ///
    /// Returns `true` if a merge was performed, and `false` if the they were already in the same set.
    pub fn join_subsets(&mut self, i: usize, j: usize) -> bool {
        let i = self.find_root_and_compress(i);
        let j = self.find_root_and_compress(j);
        if i == j {
            false
        } else {
            // Otherwise we wish to merge. Which set is larger?
            let si = self.size_of(i);
            let sj = self.size_of(j);

            // Keep the larger one as the root, since more things currently point to it.
            let (from, into) = if si < sj { (i, j) } else { (j, i) };
            self.set_parent(from, into);
            self.set_size(into, si + sj);

            self.num_sets -= 1;
            true
        }
    }

    /// Returns an iterator over the root indices of this union's subsets along with their sizes.
    pub fn sizes(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.map.len()).filter_map(|i| match self.entry(i) {
            Entry::Root { size } => Some((i, size)),
            Entry::Child { next: _ } => None,
        })
    }
}
