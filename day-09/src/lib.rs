mod graph;

use graph::Graph;


pub fn run_1(data: &Vec<String>) -> Result<usize, String> {

    let graph = Graph::new_from_paths(data)?;
    Ok(graph.find_mst_length())

}