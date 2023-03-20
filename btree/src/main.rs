#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum Node {
    None,
    Link {item: i32, children: Box<[Node; 2]> }
}

impl Node {
    pub fn new_node(&mut self, data: i32) {
        match self {
            Self::None => {
                self.to_link(data, Box::new([Self::None, Self::None]));
                return;
            },
            Self::Link{item: internal_data, children} => {
                if data < *internal_data {
                    children[0].new_node(data);
                    return;
                }
                children[1].new_node(data);
            }
        }
    }
    fn to_link(&mut self, data: i32, children: Box<[Node; 2]>) { 
        match self {
            Self::None => {
                *self = Self::Link { item: data, children };   
                return;
            },
            Self::Link{..} => {
                return;
            }
        };
    }
    fn to_none(&mut self) {
        match self {
            Self::None => {
                return;
            },
            Self::Link {..} => {
                *self = Self::None {};
            }
        }
    }
    pub fn print_nodes(&self) {
        match self {
            Self::None => {
                println!("Empty");
                return;
            },
            Self::Link{item: internal_data, children} => {
                println!("{internal_data}");
                children[0].print_nodes(); 
                children[1].print_nodes();
            }
        }
    }
    pub fn delete_node(&mut self, to_delete: i32) -> bool {
        match self {
            Self::None => return false,
            Self::Link{item: data, children} => {
                if to_delete == *data {
                    if matches!(children[0], Self::None) && matches!(children[1], Self::None) {
                        self.to_none();  
                        println!("No Children");
                    }
                    else if matches!(children[0], Self::None) ^ matches!(children[1], Self::None) {
                        self.delete_leaf_single_child();
                    }
                    else {
                        self.delete_leaf_double_child();
                    }
                    return true;
                }

                if to_delete < *data {
                    return children[0].delete_node(to_delete);
                }
                return children[1].delete_node(to_delete);
            }
        }
    }
    fn delete_leaf_single_child(&mut self) {
        println!("Single Child");
    }
    fn delete_leaf_double_child(&mut self) {
        println!("Two Children");
    }
    pub fn find_node(&self, to_find: i32) -> Result<&Node, String> {
        match self {
            Self::None => {
                return Err("couldn't find node".to_string()) ;
            },
            Self::Link { item, children } => {
                if to_find == *item {
                    return Ok(self);
                }
                if to_find < *item {
                    return children[0].find_node(to_find);
                }
                return children[1].find_node(to_find); 
            }
        }
    }
}

fn main() {
    let mut head = Node::None;
    head.new_node(23);
    head.new_node(19);
    head.new_node(69);
    head.new_node(43);
    head.new_node(90);
    head.new_node(55);
    head.print_nodes();
    
    head.delete_node(19);
}
