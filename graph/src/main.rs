#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

struct Graph {
    nodes: HashMap<char, Node>
}

#[derive(Debug)]
struct Node {
    name: char,
    connections: Box<Vec<Link>>
}

#[derive(Debug)]
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
}

fn main() {

}
