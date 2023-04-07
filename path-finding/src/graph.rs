use std::collections::HashMap;

//Directed graph
pub struct Graph {
    pub nodes: HashMap<char, Node>
}

pub struct Node {
    pub name: char,
    pub connections: Box<Vec<Link>>
}

pub struct Link {
    pub cost: i32,
    pub destination: char
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new()
        }
    }
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
    pub fn new_connection(&mut self, node_one_name: char, node_two_name: char, cost: i32) -> Result<(), &str> { 
        if node_one_name == node_two_name || !self.nodes.contains_key(&node_two_name){
            return Err("Invalid connection");
        }

        match self.nodes.get_mut(&node_one_name) {
            Some(node) => node.connections.push(Link{cost, destination: node_two_name}),
            None =>  return Err("Node does not exist") 
        }

         return Ok(());
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
    //TODO: Fix this
    pub fn delete_node(&mut self, node_name: char) -> Result<(), String> {
        let connections = match self.nodes.get(&node_name) {
            Some(node) => node.connections
                .iter()
                .map(|link| link.destination)
                .collect::<Vec<char>>(),
            None => return Err(format!("Node {} does not exist", node_name))
        }; 
        
        match self.nodes.remove(&node_name) {
            Some(_node) => return Ok(()),
            None => return Err(format!("Node {} does not exist", node_name))
        }


    }
}


