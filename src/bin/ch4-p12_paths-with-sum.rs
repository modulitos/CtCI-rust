// Paths with Sum: You are given a binary tree in which each node
// contains an integer value (which might be positive or negative).
// Design an algorithm to count the number of paths that sum to a
// given value. The path does not need to start or end at the root or
// a leaf, but it must go downwards (traveling only from parent nodes
// to child nodes).

// clarifying questions:
// space requirements? Can we use a hash table of size O(n)?

use cracking::{BTree as Tree, BTreeNode as Node, BinaryTree};
extern crate num_traits;

pub trait PathsWithSum<T> {
    fn find_paths(&self) -> u32;
}

impl<T> PathsWithSum<T> for BinaryTree<T> {
    fn find_paths(&self) -> u32 {
        0
    }
}

fn count_paths_from_node<T>(node: &Tree<T>, sum: i8) -> i8
where
    T: Copy,
    i8: std::convert::From<T>,
{
    if node.is_none() {
        return 0;
    }
    let n = node.as_ref().unwrap();

    let mut counts = 0;
    let data = i8::from(n.data);
    if data == sum {
        counts += 1;
    }
    // traverse down the node's children, counting the paths that hit
    // the sum:
    counts += count_paths_from_node(&n.left, sum - data);
    counts += count_paths_from_node(&n.right, sum - data);
    counts
}

#[test]
fn test_count_paths_from_node_simple() {
    let node = Node::<i8>::new(
        0,
        Node::new(1, None, Node::new(2, None, None)),
        Node::new(-2, None, None),
    );
    assert_eq!(count_paths_from_node(&node, 3), 1);
}

#[test]
fn test_count_paths_from_node_full_tree() {
    let node_fixture = Node::new(
        5,
        Node::new(3, Node::new(3, None, None), Node::new(-2, None, None)),
        Node::new(1, None, Node::new(2, None, None)),
    );
    assert_eq!(count_paths_from_node(&node_fixture, 8), 2);
}

#[test]
fn test_brute_force_full_tree() {
    // let node_fixture = Node::<i8>::new(
    //     10,
    //     Node::new(
    //         5,
    //         Node::new(3, Node::new(3, None, None), Node::new(-2, None, None)),
    //         Node::new(1, None, Node::new(2, None, None)),
    //     ),
    //     Node::new(-3, None, Node::new(11, None, None)),
    // );

}
