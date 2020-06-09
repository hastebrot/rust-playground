use std::cmp::max;

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Box::new(Node::new(2)));

    println!("root = {:?}", root);
    println!("depth = {}", calculate_depth(Some(Box::new(root)), 0));
}

fn calculate_depth<T>(edge: Edge<T>, depth: i32) -> i32 {
    match edge {
        None => depth,
        Some(node) => max(
            calculate_depth(node.left, depth + 1),
            calculate_depth(node.right, depth + 1),
        ),
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Edge<T>,
    right: Edge<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

type Edge<T> = Option<Box<Node<T>>>;
