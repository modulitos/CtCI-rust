// Paths with Sum: You are given a binary tree in which each node
// contains an integer value (which might be positive or negative).
// Design an algorithm to count the number of paths that sum to a
// given value. The path does not need to start or end at the root or
// a leaf, but it must go downwards (traveling only from parent nodes
// to child nodes).

// clarifying questions:
// space requirements? Can we use a hash table of size O(n)?

use cracking::{BTree as Tree, BTreeNode as Node};

// Counts paths adding to the sum, starting at the given node:
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

fn count_paths<T>(node: &Tree<T>, sum: i8) -> i8
where
    T: Copy,
    i8: std::convert::From<T>,
{
    if node.is_none() {
        return 0;
    }
    let n = node.as_ref().unwrap();

    let mut counts = 0;

    // count all paths from this node:
    counts += count_paths_from_node(node, sum);

    // count all paths from the nodes below this node:
    counts += count_paths(&n.left, sum);
    counts += count_paths(&n.right, sum);
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
fn test_count_paths_simple() {
    let node_fixture = Node::new(
        5,
        Node::new(
            3,
            Node::new(3, Node::new(2, None, None), None),
            Node::new(-2, None, None),
        ),
        Node::new(1, None, Node::new(2, None, None)),
    );
    assert_eq!(count_paths(&node_fixture, 8), 3);
}

#[test]
fn test_count_paths() {
    let node_fixture = Node::<i8>::new(
        10,
        Node::new(
            5,
            Node::new(3, Node::new(3, None, None), Node::new(-2, None, None)),
            Node::new(1, None, Node::new(2, None, None)),
        ),
        Node::new(-3, None, Node::new(11, None, None)),
    );
    assert_eq!(count_paths(&node_fixture, 8), 3);
}
