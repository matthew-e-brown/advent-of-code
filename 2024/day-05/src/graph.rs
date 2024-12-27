#![allow(unused)]

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
            graph.get_or_insert_mut(b).push_incoming(a);
        }

        graph
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    num: PageNum,
    incoming: Vec<PageNum>,
    outgoing: Vec<PageNum>,
}

impl Page {
    pub fn new(num: PageNum) -> Self {
        Page {
            num,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }

    pub fn num(&self) -> PageNum {
        self.num
    }

    pub fn incoming(&self) -> &[PageNum] {
        &self.incoming
    }

    pub fn outgoing(&self) -> &[PageNum] {
        &self.outgoing
    }

    pub fn has_incoming(&self, num: &PageNum) -> bool {
        self.incoming.binary_search(&num).is_ok()
    }

    pub fn has_outgoing(&self, num: &PageNum) -> bool {
        self.outgoing.binary_search(&num).is_ok()
    }

    pub fn push_incoming(&mut self, num: PageNum) {
        let i = self.incoming.binary_search(&num).unwrap_or_else(|i| i);
        self.incoming.insert(i, num);
    }

    pub fn push_outgoing(&mut self, num: PageNum) {
        let i = self.outgoing.binary_search(&num).unwrap_or_else(|i| i);
        self.outgoing.insert(i, num);
    }

    pub fn remove_incoming(&mut self, num: &PageNum) -> bool {
        if let Ok(i) = self.incoming.binary_search(&num) {
            self.incoming.remove(i);
            true
        } else {
            false
        }
    }

    pub fn remove_outgoing(&mut self, num: &PageNum) -> bool {
        if let Ok(i) = self.outgoing.binary_search(&num) {
            self.outgoing.remove(i);
            true
        } else {
            false
        }
    }
}
