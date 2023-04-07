use std::collections::HashMap;
use crate::graph::Graph;
use crate::dijkstra;
use crate::bellman_ford;

#[derive(Clone, Copy, Debug)]
pub struct ShortestPathEntry {
    pub cost: i32,
    pub prev_node: char,
    pub node: char
}
impl ShortestPathEntry {
    pub fn new(cost: i32, prev_node: char, node: char) -> ShortestPathEntry {
        ShortestPathEntry {
            cost,
            prev_node,
            node
        }
    } 
    pub fn to_string(&self) -> String {
        let mut to_return = "Node: ".to_string() + &self.node.to_string();
        to_return += &(" Cost: ".to_string() + &self.cost.to_string());
        to_return += &(" Prev Node: ".to_string() + &self.prev_node.to_string());
        return to_return;
    }
}

//Takes a graph, a start and destination node and returns the shortest path between them, using
//dijkstra's algorithm
pub fn find_shortest_path_dijkstra(start_node: char, destination_node: char, graph: &Graph) -> Result<String, String> {
    let tree = dijkstra::gen_shortest_path_tree(graph, start_node)?;
    return find_shortest_path(destination_node, tree);
}

//Takes a graph, a start and destination node and returns the shortest path between them, using
//bellman ford's algorithm
pub fn find_shortest_path_bellman_ford(start_node: char, destination_node: char, graph: &Graph) -> Result<String, String> {
    let tree = bellman_ford::gen_shortest_path_tree(graph, start_node)?;
    return find_shortest_path(destination_node, tree);
}


fn find_shortest_path(destination_node: char, tree: HashMap<char, ShortestPathEntry>) -> Result<String, String> {
    if !tree.contains_key(&destination_node) {
        return Err("Destination node doesn't exist".to_string());
    }

    let mut path = "".to_string();
    let mut current_node = match tree.get(&destination_node) {
        Some(r) => r,
        None => return Err("Couldn't find destination".to_owned())
    };

    path += &(&current_node.node.to_string());
    while current_node.prev_node != current_node.node {
        path += &" ,";
        current_node = match tree.get(&current_node.prev_node) {
            Some(r) => r,
            None => {
                let error = "Could not find a node: ".to_owned() + &current_node.prev_node.to_string();
                return Err(error);
            }
        };
        path += &current_node.node.to_string();
    }
    return Ok(path.chars().rev().collect());
}
