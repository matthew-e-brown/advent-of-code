use std::rc::Rc as StdRc;
use std::{hash::Hash, borrow::Borrow};
use std::collections::{HashMap, HashSet};


// Need to wrap Rc with a wrapper so we can implement

#[derive(Hash)]
struct Rc<T> {
    inner: StdRc<T>,
}

impl<T> Rc<T> {
    fn new(val: T) -> Self {
        Self { inner: StdRc::new(val) }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T> Borrow<T> for Rc<T> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T: PartialEq> PartialEq for Rc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: PartialEq> Eq for Rc<T> {}


// -- End of Rc wrapper


#[derive(Clone)]
struct Edge<T> {
    a: Rc<T>,
    b: Rc<T>,
    cost: usize,
}


pub struct Graph<T> where T: Eq + Hash + Clone {
    nodes: HashSet<Rc<T>>,
    edges: Vec<Edge<T>>,
}


type Parents<T> = HashMap<Rc<T>, Option<Rc<T>>>;


impl<T> Graph<T> where T: Eq + Hash + Clone {

    pub fn new_empty() -> Self {
        Self { nodes: HashSet::new(), edges: vec![] }
    }


    pub fn add_edge(&mut self, a: T, b: T, cost: usize) {

        let mut insert_or_get = |v: T| {
            match self.nodes.get(&v) {
                Some(rc) => rc.clone(),
                None => {
                    let rc = Rc::new(v);
                    self.nodes.insert(rc.clone());
                    rc
                },
            }
        };

        let a = insert_or_get(a);
        let b = insert_or_get(b);

        self.edges.push(Edge { a, b, cost });
        self.edges.sort_unstable_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
    }


    fn find_absolute_parent(parents: &Parents<T>, node: Rc<T>) -> Rc<T> {
        match parents.get(&node).unwrap() {
            // If you have no parent, you are the absolute parent
            None => node,
            // If you do have a parent, find its parent
            Some(next) => Self::find_absolute_parent(parents, next.clone()),
        }
    }


    pub fn find_mst_length(&self) -> usize {

        let mut total = 0;
        let mut count = 0;

        let mut parents: Parents<T> = {
            let mut map = HashMap::new();
            for node in self.nodes.iter() { map.insert(node.clone(), None); }
            map
        };

        // We don't need to sort because the add_edges function keeps them sorted

        let mut iter = self.edges.iter();

        while count < self.nodes.len() {
            let Edge { a, b, cost } = iter.next().unwrap();
            let parent_a = Self::find_absolute_parent(&parents, a.clone());
            let parent_b = Self::find_absolute_parent(&parents, b.clone());

            // If they have different absolute parents, we have not found a cycle
            if parent_a != parent_b {

                // We set the parent of 'b' to be 'a'
                *parents.get_mut(a).unwrap() = Some(b.clone());

                // We add to our total length and our count
                total += cost;
                count += 1;

            }
        }

        total
    }

}


use lazy_static::lazy_static;
use regex::Regex;

impl Graph<String> {

    pub fn new_from_paths(paths: &Vec<String>) -> Result<Self, String> {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+)\s+to\s+(\w+)\s+=\s+([\d\.]+)$").unwrap();
        }

        let mut graph = Self::new_empty();

        for path in paths.iter() {
            let caps = RE.captures(path).ok_or(format!("Malformed line: `{}`", path))?;

            // Because our Regex is strict enough, we can unwrap all the groups
            let a = caps.get(1).unwrap().as_str().to_owned();
            let b = caps.get(2).unwrap().as_str().to_owned();
            let cost = caps.get(3).unwrap().as_str().parse().unwrap();

            graph.add_edge(a, b, cost);
        }

        Ok(graph)
    }

}