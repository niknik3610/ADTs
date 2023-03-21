#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

struct Graph {
    nodes: HashMap<char, Node>
}

struct Node {
    name: char,
    connections: Box<Vec<Link>>
}

struct Link {
    cost: u32,
    destination: char
}

impl Graph {
    pub fn new_node(&mut self, name: char) -> bool {
        match self.nodes.get(&name) {
            Some(_node) => return false,
            None => {}
        }

        let new_node = Node {
            name, 
            connections: Box::new(Vec::new()) 
        };
        self.nodes.insert(name, new_node);
        return true;
    }
    pub fn new_connection(&mut self, node_one_name: char, node_two_name: char, cost: u32) -> bool { 
        if node_one_name == node_two_name {
            return false;
        }

        match self.nodes.contains_key(&node_two_name) {
            true => {}
            false => return false
        }

        match self.nodes.get_mut(&node_one_name) {
            Some(node) => node.connections.push(Link{cost, destination: node_two_name}),
            None => return false
        }

        match self.nodes.get_mut(&node_two_name) {
            Some(node) => node.connections.push(Link{cost, destination: node_one_name}),
            None => return false
        } 
        return true;
    }
    pub fn print_graph(&self) {
        println!("-----------------------------------------------------");
        for (key, node) in &self.nodes {
            println!("Node: {key}");
            println!("Connections:");  
            for link in node.connections.as_slice() {
                println!("Name: {} Cost: {}", link.destination, link.cost);
            } 
            println!();
        }
    }
}

fn main() {
    let mut graph = Graph {
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
}
