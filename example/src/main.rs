use std::fmt;
struct Node {
    data: char,
    node: Option<Box<Node>>,
}

impl Node {
    fn new(data: char, node: Option<Box<Node>>) -> Self {
        Self { data, node }
    }
}

struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, data: char) {
        let new_node = Box::new(Node::new(data, self.head.take()));
        self.head = Some(new_node);
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.data);
                    current = &node.node;
                }
                None => break,
            }
        }
        write!(f, "{}\n", result)
    }
}

fn main() {
    let mut list: Stack = Stack::new();
    let world: [char; 5] = ['o', 'l', 'l', 'e', 'h'];
    for chr in world.iter() {
        list.push(*chr)
    }
    println!("{}", list);
    // std:smart_ptr
    // let x = Box::new(10);
    // println!("{}", *x)
}
