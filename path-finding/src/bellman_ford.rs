use crate::graph::*;
use crate::path_finder::ShortestPathEntry;
use std::collections::HashMap;

const INFINITY: i32 = 99999;

struct Edge {
    origin: char,
    destination: char,
    cost: i32
}

type ShortestPathTree = HashMap<char, ShortestPathEntry>;
pub fn gen_shortest_path_tree(graph: &Graph, start_node: char) -> Result<ShortestPathTree, String> { 
    let mut shortest_path_tree: ShortestPathTree = HashMap::new(); 
    let mut edges: Vec<Edge> = Vec::new();

    //populate distance 
    for (.., node) in graph.nodes.iter() {
        shortest_path_tree.insert(node.name, ShortestPathEntry::new(INFINITY, '$', node.name));
        for edge in node.connections.as_slice() {
            edges.push(Edge{origin: node.name, destination: edge.destination, cost: edge.cost});
        }
    }

    match shortest_path_tree.get_mut(&start_node) {
        Some(entry) => {
            entry.cost = 0;
            entry.prev_node = start_node;
        }
        None => return Err("This start node doesn't exist".to_string())
    }

    for _ in 0..shortest_path_tree.len() - 1 {
        for edge in edges.as_slice() {
            let origin = shortest_path_tree.get(&edge.origin).expect("Error: Expected a cost of node").clone();
            let mut destination = shortest_path_tree.get_mut(&edge.destination).expect("Error: Expected a cost of node");
            if origin.cost + edge.cost < destination.cost {
                destination.cost = origin.cost + edge.cost;
                destination.prev_node = edge.origin;
            }
        }
    }

    return Ok(shortest_path_tree);
}
