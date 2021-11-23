use std::{rc::Rc, borrow::Borrow, hash::{Hash, Hasher}};
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Hash)]
struct GraphNode<'a> {
    alias: &'a str,
}


// https://stackoverflow.com/a/57931787/10549827

#[derive(Debug, PartialEq, Eq, Hash)]
struct RcNode<'a>(Rc<GraphNode<'a>>);

impl<'a> RcNode<'a> {
    fn new(alias: &'a str) -> Self {
        // create a new RcNode by making a new Rc and returning Self { 0: } as a wrapper
        Self { 0: Rc::new(GraphNode { alias }) }
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


#[derive(Debug, PartialEq, Eq)]
struct GraphEdge<'a> {
    source: RcNode<'a>,
    target: RcNode<'a>,
    cost: usize
}

impl<'a> Hash for GraphEdge<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.target.hash(state);
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
            let source = if let Some(rc) = nodes.get(&source) {
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
            let target = if let Some(rc) = nodes.get(&target) {
                rc.clone()
            } else {
                let new_rc = RcNode::new(target);
                nodes.insert(new_rc.clone());
                new_rc
            };

            let new_edge = GraphEdge { source, target, cost };
            if edges.contains(&new_edge) {
                return Err(format!("Duplicate line: `{}`", string));
            } else {
                edges.insert(new_edge);
            }
        }

        Ok(Graph { nodes, edges })
    }

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