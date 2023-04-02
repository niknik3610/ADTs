use std::collections::HashMap;
use crate::graph::ShortestPathEntry;
use crate::graph::Graph;

const INFINITY: i32 = 99999;

type ShortestPathTree = HashMap<char, ShortestPathEntry>;
pub fn gen_shortest_path_tree(graph: &Graph, start_node: char) -> Result<ShortestPathTree, String> {
    let mut not_visited: ShortestPathTree = HashMap::new();
    let mut visited: ShortestPathTree = HashMap::new();

    for (key, ..) in &graph.nodes {
        if *key == start_node {
            not_visited.insert(start_node, ShortestPathEntry::new(0, start_node, start_node)); 
            continue;
        }
        not_visited.insert(*key, ShortestPathEntry::new(INFINITY, '$', *key));
    }
    
    let temp_path = ShortestPathEntry::new(INFINITY+1, '$', '$');

    loop {
        let mut smallest_value = temp_path.clone();
        let mut found_a_value = false;
        for (_key, node) in not_visited.iter() {
            if node.cost < smallest_value.cost {
                smallest_value = node.clone();
                found_a_value = true;
            }
        } 
        if !found_a_value {
            break;
        }

        not_visited.remove(&smallest_value.node);
        visited.insert(smallest_value.node, smallest_value);

        for connection in graph.nodes.get(&smallest_value.node).unwrap().connections.as_slice() {
            if connection.cost < 0 {
                return Err("Connection cost smaller than zero".to_string());
            }

            match not_visited.get_mut(&connection.destination) {
                None => {}
                Some(node) => {
                    let new_cost = connection.cost + smallest_value.cost;
                    if node.cost > new_cost {
                        node.cost = connection.cost + smallest_value.cost;
                        node.prev_node = smallest_value.node;
                    }
                }
            }
        }
    }
    return Ok(visited);
}

pub fn find_shortest_path(destination_node: char, tree: HashMap<char, ShortestPathEntry>) -> Result<String, String> {
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

