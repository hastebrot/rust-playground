use std::cmp::max;

fn main() {
    let mut root = Vertex::new(1);
    root.left = Some(Box::new(Vertex::new(2)));

    println!("root = {:?}", root);
    println!("depth = {}", calculate_depth(Some(Box::new(root)), 0));
}

fn calculate_depth<T>(link: Link<T>, depth: i32) -> i32 {
    match link {
        None => depth,
        Some(node) => max(
            calculate_depth(node.left, depth + 1),
            calculate_depth(node.right, depth + 1),
        ),
    }
}

#[derive(Debug)]
struct Vertex<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Vertex<T> {
    fn new(value: T) -> Self {
        Vertex {
            value,
            left: None,
            right: None,
        }
    }
}

type Link<T> = Option<Box<Vertex<T>>>;
