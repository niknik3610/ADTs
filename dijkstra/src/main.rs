mod graph;
use graph::Graph;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

const INFINITY: u32 = 99999;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct ShortestPathEntry {
    node: char,
    cost: u32
}

fn shortest_path(graph: &Graph, start_node: char, end_node: char) -> Result<String, String> {
    if !graph.nodes.contains_key(&start_node) || !graph.nodes.contains_key(&end_node) {
        return Err("Graph doesn't contain these nodes".to_string()); 
    }
    let mut not_visited = HashMap::new();
    let mut visited = HashSet::new();
    let mut final_path: BTreeSet<char> = BTreeSet::new();

    for (key, ..) in &graph.nodes {
        if *key == start_node {
            not_visited.insert(start_node, 0);
            continue;
        }
        not_visited.insert(*key, INFINITY );
    }
    
    let temp_path = ShortestPathEntry {
        node: '$',
        cost: INFINITY + 1
    }; 

    loop {
        println!("visited: {visited:?}");
        println!("not visited: {not_visited:?}"); 

        let mut smallest_value = temp_path.clone();
        for (node, cost) in not_visited.iter() {
            if cost < &smallest_value.cost {
                smallest_value = ShortestPathEntry {node: *node, cost: *cost};
            }
        } 
        if smallest_value == temp_path  {
            break;
        }

        not_visited.remove(&smallest_value.node);
        visited.insert(smallest_value);

        for entry in graph.nodes.get(&smallest_value.node).unwrap().connections.as_slice() {
            match not_visited.get_mut(&entry.destination) {
                None => {}
                Some(cost) => *cost = entry.cost + smallest_value.cost
            }
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

    shortest_path(&graph, 'A', 'C').unwrap();
}
