mod graph;
use graph::Graph;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

const INFINITY: u32 = 99999;

#[derive(Hash, Eq, PartialEq, Debug)]
struct ShortestPathEntry {
    node: char,
    cost: u32
}

fn shortest_path(graph: &Graph, start_node: char, end_node: char) -> Result<String, String> {
    if !graph.nodes.contains_key(&start_node) || !graph.nodes.contains_key(&end_node) {
        return Err("Graph doesn't contain these nodes".to_string()); 
    }
    let mut distances = HashSet::new();
    let mut spt = HashSet::new();
    let mut final_path: BTreeSet<char> = BTreeSet::new();

    for (key, ..) in &graph.nodes {
        if *key == start_node {
            distances.insert(ShortestPathEntry {node: start_node, cost: 0});
            final_path.insert(*key);
            continue;
        }
        distances.insert(ShortestPathEntry { node: *key, cost: INFINITY });
    }
    for entry in distances {
        println!("Node Name: {}, Node Cost: {}", entry.node, entry.cost);
    }
    
    let temp_path = ShortestPathEntry {
        node: '$',
        cost: INFINITY
    }; 

    loop {
        let smallest_value = temp_path;
        for entry in distances {
            
        }
    }

    return Ok("Temp Entry".to_string());
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

    shortest_path(&graph, 'A', 'B').unwrap();
}
