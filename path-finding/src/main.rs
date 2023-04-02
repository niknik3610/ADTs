#![allow(dead_code)]

use std::collections::HashMap;
use crate::dijkstra::*;

mod dijkstra;
mod graph;
mod bellman_ford;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() { 
        let mut graph = graph::Graph::new();                
        graph.new_node('C');
        graph.new_node('D');
        //graph.print_graph();

        graph.new_connection('A', 'B', 5);
        graph.new_connection('B', 'C', 10);
        graph.new_connection('C', 'D', 3);
        graph.new_connection('D', 'A', 4);
        //graph.print_graph();

        let path_tree = match gen_shortest_path_tree(&graph, 'A') {
            Ok(r) => r,
            Err(e) => {
                panic!("{e}");
            }
        }; 

        println!("{:?}", path_tree);

        let shortest_path = match find_shortest_path('D', path_tree) {
            Ok(r) => r,
            Err(e) => {
                panic!("{e}");
            }
        };    

        assert_eq!("A, B, C, D", shortest_path);
    }
    #[test]
    fn test_bellman_ford() {
        let mut graph = graph::Graph {
            nodes: HashMap::new()
        };
        graph.new_node('A');
        graph.new_node('B');
        graph.new_node('C');
        graph.new_node('D');
        //graph.print_graph();

        graph.new_connection('A', 'B', 5);
        graph.new_connection('B', 'C', 10);
        graph.new_connection('C', 'D', 3);
        graph.new_connection('D', 'A', 4);
        //graph.print_graph();
        let path_tree = match bellman_ford::gen_shortest_path_tree(graph, 'A') {
            Ok(r) => r,
            Err(e) => { 
                panic!("{e}");
            }
        };
 
        let shortest_path = match dijkstra::find_shortest_path('D', path_tree) {
            Ok(r) => r,
            Err(e) => {
                panic!("{e}");
            }
        }; 

        assert_eq!("A, B, C, D", shortest_path);
    }
}

fn main() {
    println!("Dijkstra:");

    println!("Bellman Ford:");

}
