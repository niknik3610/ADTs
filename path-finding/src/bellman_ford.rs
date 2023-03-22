use crate::graph::*;
use std::collections::HashMap;

const INFINITY: i32 = 99999;

struct ShortestPathEntry {
    cost: i32,
    prev_node: char
}

struct Edge {
    origin: char,
    destination: char,
    cost: i32
}

pub fn gen_shortest_path_tree(graph: Graph, start_node: char) -> Result<String, String> {
    let mut distances: HashMap<char, i32> = HashMap::new();  
    let mut edges: Vec<Edge> = Vec::new();

    //populate distance 
    for (.., node) in graph.nodes {
        distances.insert(node.name, INFINITY);
        for edge in node.connections.as_slice() {
            edges.push(Edge{origin: node.name, destination: edge.destination, cost: edge.cost});
        }
    }
    match distances.get_mut(&start_node) {
        Some(cost) => *cost = 0,
        None => return Err("This start node doesn't exist".to_string())
    }
    
    for _i in 0..distances.len()-1 {
        for edge in edges.as_slice() {
            if *distances.get(&edge.origin).unwrap() + edge.cost < *distances.get(&edge.destination).unwrap() {
                *distances.get_mut(&edge.destination).unwrap() = 
                    *distances.get(&edge.origin).unwrap() + edge.cost;
            }
        }
    }

    for (node, cost) in distances {
        println!("Node: {node} Cost: {cost}");
    }

    return Ok("temp".to_string()); 
}
