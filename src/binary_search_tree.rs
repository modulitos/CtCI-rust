use std::mem;

// This implementation is largely inspired by:
// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/e79494a07c8d771e0d357ed05eb6d7ddb58a3bf8/Chapter05/src/binary_search_tree.rs

type Tree<T> = Option<Box<Node<T>>>;
struct Node<T> {
    pub data: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Tree<T> {
        Some(Box::new(Node {
            data,
            left: None,
            right: None,
        }))
    }
}

pub struct BinarySearchTree<T> {
    root: Tree<T>,
    pub length: u64,
}

impl<T> BinarySearchTree<T>
where
    T: std::cmp::PartialOrd,
{
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, data: T) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, data);
    }

    fn add_rec(&mut self, node: Tree<T>, data: T) -> Tree<T> {
        match node {
            Some(mut n) => {
                if n.data <= data {
                    n.left = self.add_rec(n.left, data);
                } else {
                    n.right = self.add_rec(n.right, data);
                }
                Some(n)
            }
            _ => Node::new(data),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_bst() {
        let _ = BinarySearchTree::<u32>::new();
        assert!(true);
    }
}
