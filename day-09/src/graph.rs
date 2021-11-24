// https://opendatastructures.org/ods-python/12_Graphs.html
// https://en.wikipedia.org/wiki/Edmonds%27_algorithm
// https://stackoverflow.com/a/57931787/10549827


use std::{borrow::Borrow, fmt::Display, hash::{Hash, Hasher}, rc::Rc};
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GraphNode<'a> {
    alias: &'a str,
}


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RcNode<'a>(Rc<GraphNode<'a>>);

impl<'a> RcNode<'a> {
    fn new(alias: &'a str) -> Self {
        // create a new RcNode by making a new Rc and returning Self { 0: } as a wrapper
        Self { 0: Rc::new(GraphNode { alias }) }
    }
}

impl<'a> Display for RcNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.alias)
    }
}

impl<'a> Clone for RcNode<'a> {
    fn clone(&self) -> Self {
        // To clone an RcNode, we actually clone the Rc reference, meaning no data is copied; we nest the cloning of the
        // shared reference.
        Self { 0: self.0.clone() }
    }
}

impl<'a> Borrow<&'a str> for RcNode<'a> {
    fn borrow(&self) -> &&'a str {
        &self.0.alias
    }
}

impl<'a> Borrow<Rc<GraphNode<'a>>> for RcNode<'a> {
    fn borrow(&self) -> &Rc<GraphNode<'a>> {
        &self.0
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct GraphEdge<'a> {
    node_a: RcNode<'a>,
    node_b: RcNode<'a>,
    cost: usize
}

impl<'a> Hash for GraphEdge<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_a.hash(state);
        self.node_b.hash(state);
    }
}

impl<'a> Ord for GraphEdge<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort a graph edge based on their costs/weights
        self.cost.cmp(&other.cost)
    }
}


#[derive(Debug)]
pub struct Graph<'a> {
    nodes: HashSet<RcNode<'a>>,
    edges: HashSet<GraphEdge<'a>>,
}


impl<'a> Graph<'a> {

    pub fn new(data: &'a Vec<String>) -> Result<Self, String> {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+)\s+to\s+(\w+)\s+=\s+([\d\.]+)$").unwrap();
        }

        let mut nodes: HashSet<RcNode> = HashSet::new();
        let mut edges: HashSet<GraphEdge> = HashSet::new();

        for string in data.iter() {
            let caps = RE.captures(string).ok_or(format!("Poorly formatted line: `{}`", string))?;

            let source = caps.get(1).unwrap().as_str();
            let target = caps.get(2).unwrap().as_str();
            let cost = caps.get(3).unwrap().as_str().parse().unwrap();

            // Check if the source node already exists
            let node_a = if let Some(rc) = nodes.get(&source) {
                // Get the reference to the existing node
                rc.clone()
            } else {
                // Make a new node and get a reference to it. We copy the reference, putting on in the 'nodes' set and
                // will put the other in the 'edges' set.
                let new_rc = RcNode::new(source);
                nodes.insert(new_rc.clone());
                new_rc
            };

            // Do the same for the target
            let node_b = if let Some(rc) = nodes.get(&target) {
                rc.clone()
            } else {
                let new_rc = RcNode::new(target);
                nodes.insert(new_rc.clone());
                new_rc
            };

            // Sort them so our Hash will match duplicates for undirected graph (L->R == R->L)
            let mut sort_me = vec![node_a, node_b];
            sort_me.sort();

            let new_edge = GraphEdge { node_a: sort_me[0].clone(), node_b: sort_me[1].clone(), cost };
            if edges.contains(&new_edge) {
                return Err(format!("Duplicate pair: `{}` <-> `{}`", sort_me[0], sort_me[1]));
            } else {
                edges.insert(new_edge);
            }
        }

        Ok(Graph { nodes, edges })
    }


    /*
    fn detect_cycle(edges: &Vec<&GraphEdge>, new_edge: &GraphEdge) -> bool {

        let mut visited = HashMap::new();
        for edge in edges.iter() { visited.insert(*edge, false); }
        visited.insert(new_edge, false);

    }


    pub fn mst(&self) -> HashSet<&GraphEdge> {

        // Sort all edges in ascending order by their weight
        let sorted = {
            let mut vec: Vec<&GraphEdge> = self.edges.iter().collect();
            vec.sort();
            vec
        };

        let sorted = sorted.iter();

        // Construct a spanning tree out of all nodes that don't create a cycle
        let mut spanning = Vec::new();

        while spanning.len() < self.nodes.len() - 1 {

            // Can unwrap because recursive algorithm is guaranteed
            let edge = *sorted.next().unwrap();

            if !Self::detect_cycle(&spanning, edge) {
                spanning.push(edge);
            }

        }

        HashSet::from_iter(spanning)
    }
    */

}


#[cfg(test)]
mod tests {

    use super::*;

    fn example_data() -> Vec<String> {
        vec![
            "London to Dublin = 464".to_owned(),
            "London to Belfast = 518".to_owned(),
            "Dublin to Belfast = 141".to_owned(),
        ]
    }


    #[test]
    fn generates() {

        let data = example_data();
        let graph = Graph::new(&data).unwrap();

        println!("{:#?}", graph);

    }

}