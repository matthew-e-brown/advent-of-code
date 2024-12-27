/// Type alias used to mark page numbers.
type PageNum = u32;

/// Parses two page numbers separated by a pipe (`|`). `a|b` means that `a` must come before `b` in the page ordering.
/// In other words, `a|b` means that there is an edge `a -> b` in the dependency graph.
fn parse_edge(line: &str) -> (PageNum, PageNum) {
    let mut a = None;
    let mut b = None;

    for s in line.split('|') {
        let p = s.parse::<u32>().expect("page dependency rule should contain valid u32");
        if a.is_none() {
            a = Some(p);
        } else if b.is_none() {
            b = Some(p);
        } else {
            panic!("page dependency rule should only have two u32");
        }
    }

    let a = a.expect("page dependency rule should have at least two u32");
    let b = b.expect("page dependency rule should have at least two u32");
    (a, b)
}

#[derive(Clone, Debug, Default)]
pub struct PageGraph {
    pages: Vec<Page>,
}

impl PageGraph {
    pub fn get(&self, num: &PageNum) -> Option<&Page> {
        self.pages.binary_search_by_key(num, |p| p.num).ok().map(|i| &self.pages[i])
    }

    pub fn get_or_insert_mut(&mut self, num: PageNum) -> &mut Page {
        match self.pages.binary_search_by_key(&num, |p| p.num) {
            Ok(i) => &mut self.pages[i],
            Err(i) => {
                self.pages.insert(i, Page::new(num));
                &mut self.pages[i]
            },
        }
    }

    pub fn from_input<I, S>(lines: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut graph = PageGraph::default();

        for line in lines {
            let (a, b) = parse_edge(line.as_ref());
            graph.get_or_insert_mut(a).push_outgoing(b);
        }

        graph
    }

    /// Subsets this graph to contain only the provided nodes.
    pub fn subset(&self, pages: &[PageNum]) -> Self {
        // Copy to a sorted version
        let keep_pages = {
            let mut v = pages.into_iter().copied().collect::<Vec<_>>();
            v.sort();
            v
        };

        // Start from an empty graph:
        let mut dst = PageGraph::default();
        for &keep_num in &keep_pages {
            // If the original graph contained the node we want to keep, create a new node with the same number, and copy over only the
            if let Some(keep) = self.get(&keep_num) {
                let mut clone = Page::new(keep_num);
                clone.outgoing = keep
                    .outgoing()
                    .into_iter()
                    .filter(|p| keep_pages.binary_search(p).is_ok())
                    .copied()
                    .collect();
                if let Err(i) = dst.pages.binary_search_by_key(&keep_num, |page| page.num()) {
                    dst.pages.insert(i, clone);
                }
            }
        }

        dst
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    num: PageNum,
    outgoing: Vec<PageNum>,
}

impl Page {
    pub fn new(num: PageNum) -> Self {
        Page { num, outgoing: Vec::new() }
    }

    pub fn num(&self) -> PageNum {
        self.num
    }

    pub fn outgoing(&self) -> &[PageNum] {
        &self.outgoing
    }

    pub fn push_outgoing(&mut self, num: PageNum) {
        let i = self.outgoing.binary_search(&num).unwrap_or_else(|i| i);
        self.outgoing.insert(i, num);
    }
}
