use std::collections::HashMap;
use crate::dijkstra::*;

mod dijkstra;
mod graph;
mod bellman_ford;

fn test_dijkstra() { 
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

    for (_key, node)in &tree {
        println!("{}", node.to_string());
    }

    let path = match find_shortest_path('D', tree) {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!("Shortest Path: {path}");
}

fn main() {
    //test_dijkstra();
}
