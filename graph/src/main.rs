#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

#[derive(Debug)]
struct Link {
    cost: u32,
    destination: Node
}

#[derive(Debug)]
struct Node {
    name: char,
    connections: Box<Vec<Link>>
}

struct Graph {
    nodes: HashMap<char, Node>
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
}

fn main() {

}
