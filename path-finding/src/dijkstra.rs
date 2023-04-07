use std::collections::HashMap;
use crate::graph::Graph;
use crate::path_finder::ShortestPathEntry;

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
