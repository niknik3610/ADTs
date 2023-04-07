#![allow(dead_code)]

use std::collections::HashMap;
use crate::dijkstra::*;

mod dijkstra;
mod graph;
mod bellman_ford;
mod path_finder;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path_finder;

    #[test]
    fn test_dijkstra() { 
        let mut graph = graph::Graph::new();                
        graph.new_node('C');
        graph.new_node('D');
        graph.new_node('A');
        graph.new_node('B');
        //graph.print_graph();

        graph.new_connection('A', 'B', 5).unwrap();
        graph.new_connection('B', 'C', 10).unwrap();
        graph.new_connection('C', 'D', 3).unwrap();
        graph.new_connection('D', 'A', 4).unwrap();
        //graph.print_graph();

        let shortest_path = match path_finder::find_shortest_path_dijkstra('A', 'D', &graph) {
            Ok(r) => r,
            Err(e) => {
                panic!("{e}");
            }
        };

        assert_eq!("A, B, C, D", shortest_path);
    }
    #[test]
    fn test_bellman_ford() {
        let mut graph = graph::Graph::new();
        graph.new_node('A');
        graph.new_node('B');
        graph.new_node('C');
        graph.new_node('D');

        graph.new_connection('A', 'B', 5).unwrap();
        graph.new_connection('B', 'C', 10).unwrap();
        graph.new_connection('C', 'D', 3).unwrap();
        graph.new_connection('D', 'A', 4).unwrap();

        let shortest_path = match path_finder::find_shortest_path_bellman_ford('A', 'D', &graph) {
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
