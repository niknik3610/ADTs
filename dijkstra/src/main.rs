mod graph;
use graph::Graph;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

const INFINITY: u32 = 99999;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct ShortestPathEntry {
    node: char,
    cost: u32,
    prev_node: char
}

impl ShortestPathEntry {
    pub fn to_string(&self) -> String {
        let mut to_return = "Node: ".to_string() + &self.node.to_string();
        to_return += &(" Cost: ".to_string() + &self.cost.to_string());
        to_return += &(" Prev Node: ".to_string() + &self.prev_node.to_string());
        return to_return;
    }
}

fn gen_shortest_path_tree(graph: &Graph, start_node: char) -> Result<HashMap<char, ShortestPathEntry>, String> {
    //if !graph.nodes.contains_key(&start_node) || !graph.nodes.contains_key(&end_node) {
    //    return Err("Graph doesn't contain these nodes".to_string()); 
    //}
    let mut not_visited: HashMap<char, ShortestPathEntry> = HashMap::new();
    let mut visited: HashMap<char, ShortestPathEntry> = HashMap::new();

    for (key, ..) in &graph.nodes {
        if *key == start_node {
            not_visited.insert(start_node, ShortestPathEntry{node: start_node, cost: 0, prev_node: start_node});
            continue;
        }
        not_visited.insert(*key, ShortestPathEntry { node: *key, cost: INFINITY, prev_node: '$' });
    }
    
    let temp_path = ShortestPathEntry {
        node: '$',
        cost: INFINITY + 1,
        prev_node: '$'
    }; 

    loop {
        let mut smallest_value = temp_path.clone();
        for (_key, node) in not_visited.iter() {
            if node.cost < smallest_value.cost {
                smallest_value = node.clone();
            }
        } 
        if smallest_value == temp_path  {
            break;
        }

        not_visited.remove(&smallest_value.node);
        visited.insert(smallest_value.node, smallest_value);

        for connection in graph.nodes.get(&smallest_value.node).unwrap().connections.as_slice() {
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

fn find_shortest_path(destination_node: char, tree: HashMap<char, ShortestPathEntry>) -> Result<String, String> {
     

}

fn main() { 
    let mut graph = graph::Graph {
        nodes: HashMap::new()
    };
    graph.new_node('A');
    graph.new_node('B');
    graph.new_node('C');
    graph.new_node('D');
    graph.print_graph();

    graph.new_connection('A', 'B', 5);
    graph.new_connection('B', 'C', 10);
    graph.new_connection('C', 'D', 3);
    graph.new_connection('D', 'A', 4);
    graph.print_graph();

    let tree = match gen_shortest_path_tree(&graph, 'C') {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    for (_key, node)in tree {
        println!("{}", node.to_string());
    }
}
