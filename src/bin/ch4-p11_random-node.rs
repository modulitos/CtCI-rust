// Random Node: You are implementing a binary tree class from scratch
// which, in addition to insert, find, and delete, has a method
// getRandomNode() which returns a random node from the tree. All
// nodes should be equally likely to be chosen. Design and implement
// an algorithm for getRandomNode, and explain how you would implement
// the rest of the methods.
use rand::{thread_rng, Rng};
use std::fmt;

type BareTree<T> = Box<Node<T>>;
type Tree<T> = Option<BareTree<T>>;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    left: Tree<T>,
    right: Tree<T>,
    length: u32,
}

impl<T> Node<T> {
    fn new(data: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        // find the length that this node will carry
        let length = 1
            + if let Some(l) = &left { l.length } else { 0 }
            + if let Some(r) = &right { r.length } else { 0 };
        Some(Box::new(Node {
            data,
            left,
            right,
            length,
        }))
    }
}

impl<T: PartialEq + fmt::Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.left == other.left && self.right == other.right
    }
}

struct BinaryTree<T> {
    root: Tree<T>,
}

impl<T> BinaryTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug + std::marker::Copy,
{
    fn new(root: Tree<T>) -> Self {
        BinaryTree { root }
    }

    fn get_length(&self) -> u32 {
        if let Some(r) = &self.root {
            r.length
        } else {
            0
        }
    }

    fn get_random_node(&self) -> Option<T> {
        if let Some(n) = &self.root {
            let mut rng = thread_rng();
            let range: u32 = rng.gen_range(0, n.length);
            Some(get_node_at_index(range, &self.root))
        } else {
            None
        }
    }
}

fn get_node_at_index<'a, T>(index: u32, node: &Tree<T>) -> T
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug + std::marker::Copy,
{
    let n = node.as_ref().unwrap();
    if index > n.length {
        panic!(
            "_get_node_at_index: invariant violated for node: {:?} at index: {}",
            n, index
        );
    }
    if index == 0 {
        return n.data;
    }
    match (&n.left, &n.right) {
        (None, None) => n.data,
        (Some(_), None) => get_node_at_index(index - 1, &n.left),
        (None, Some(_)) => get_node_at_index(index - 1, &n.right),
        (Some(l), Some(r)) => {
            if index <= l.length {
                get_node_at_index(index - 1, &n.left)
            } else {
                get_node_at_index(index - l.length - 1, &n.right)
            }
        }
    }
}

#[test]
fn create_empty() {
    let bt = BinaryTree::<u32>::new(Node::new(1, None, None));
    assert_eq!(bt.get_length(), 1);
}

#[test]
fn create_complex() {
    let bt = BinaryTree::<u32>::new(Node::new(
        1,
        Node::new(
            3,
            Node::new(4, None, None),
            Node::new(5, None, Node::new(7, None, None)),
        ),
        Node::new(2, None, None),
    ));
    assert_eq!(bt.get_length(), 6);
}

#[test]
fn test_get_node_at_index_0() {
    let node = Node::new(1, None, Node::new(2, None, None));
    assert_eq!(get_node_at_index(0, &node), node.unwrap().data);
}

#[test]
fn test_get_node_at_index_simple() {
    let node = Node::new(
        1,
        Node::new(4, None, None),
        Node::new(2, None, Node::new(3, None, None)),
    );
    assert_eq!(get_node_at_index(3, &node), 3);
}

#[test]
fn test_get_node_at_index_complex() {
    let node = Node::new(
        1,
        Node::new(4, None, None),
        Node::new(
            2,
            None,
            Node::new(
                3,
                Node::new(5, Node::new(9, None, None), None),
                Node::new(6, Node::new(7, None, None), Node::new(8, None, None)),
            ),
        ),
    );
    assert_eq!(get_node_at_index(0, &node), 1);
    assert_eq!(get_node_at_index(1, &node), 4);
    assert_eq!(get_node_at_index(2, &node), 2);
    assert_eq!(get_node_at_index(3, &node), 3);
    assert_eq!(get_node_at_index(4, &node), 5);
    assert_eq!(get_node_at_index(5, &node), 9);
    assert_eq!(get_node_at_index(6, &node), 6);
    assert_eq!(get_node_at_index(7, &node), 7);
    assert_eq!(get_node_at_index(8, &node), 8);
}

#[test]
fn test_get_random_node() {
    let node = Node::new(
        1,
        Node::new(4, None, None),
        Node::new(
            2,
            None,
            Node::new(
                3,
                Node::new(5, Node::new(9, None, None), None),
                Node::new(6, Node::new(7, None, None), Node::new(8, None, None)),
            ),
        ),
    );
    // NOTE: we are visually inspecting the output here to check
    // whether the distribution is random.
    let bt = BinaryTree::<u32>::new(node);
    let mut v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
    v = bt.get_random_node().unwrap();
    println!("v: {}", v);
    assert!((1..10).contains(&v));
}
