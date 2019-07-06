// Successor: Write an algorithm to find the "next" node (i.e.,
// in-order successor) of a given node in a binary search tree. You
// may assume that each node has a link to its parent.

// clarifying questions:
// - Return None if there is no "next" node?
// - Is it okay to use the node's value, instead of the node itself? Seems like a better abstraction...

use cracking::{RCBinarySearchTree as BinarySearchTree, RCTree as Tree};
use std::rc::Rc;

trait FindSuccessor<T> {
    fn find_successor(&self, data: T) -> Option<T>;
    fn _find_node(&self, node: Tree<T>, data: T) -> Tree<T>;
    fn _find_lowest_value(&self, node: &Tree<T>) -> Option<T>;
}

impl<T> FindSuccessor<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn find_successor(&self, data: T) -> Option<T> {
        // The lowest value in the right subtree should be "next". If
        // there is no right subtree, then return the parent.
        let node = self._find_node(self.root.clone(), data);
        if node.is_none() {
            return None;
        }
        let n = node.unwrap();

        let lowest_right = self._find_lowest_value(&n.borrow_mut().right.clone());
        if lowest_right.is_some() {
            lowest_right
        } else {
            if let Some(p) = &n.borrow().parent {
                // Check whether this node is the left or right child of the parent.
                let parent = p.borrow().clone();
                if let Some(parent_left_child) = parent.left {
                    if Rc::ptr_eq(&parent_left_child, &n) {
                        // if our node is the left child, and has no
                        // right child, then the parent is the "next"
                        // node:
                        return Some(p.borrow().data.clone());
                    }
                }
            }
            None
        }
    }

    fn _find_node(&self, node: Tree<T>, data: T) -> Tree<T> {
        match node {
            Some(n) => {
                let n_borrowed = n.borrow();
                if n_borrowed.data == data {
                    Some(n.clone())
                } else if data < n_borrowed.data {
                    self._find_node(n_borrowed.left.clone(), data)
                } else {
                    self._find_node(n_borrowed.right.clone(), data)
                }
            }
            _ => None,
        }
    }

    fn _find_lowest_value(&self, node: &Tree<T>) -> Option<T> {
        if let Some(n) = node {
            let n_borrowed = n.borrow();
            if let Some(left) = n_borrowed.left.clone() {
                self._find_lowest_value(&Some(left))
            } else {
                Some(n_borrowed.data.clone())
            }
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn find_node() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(2);
        t.add(1);
        t.add(3);
        t.add(0);
        let n = t._find_node(t.root.clone(), 2);
        assert!(n.is_some());
        assert_eq!(n.unwrap().borrow().data, 2);

        let n2 = t._find_node(t.root.clone(), 3);
        assert!(n2.is_some());
        assert_eq!(n2.unwrap().borrow().data, 3);

        let n3 = t._find_node(t.root.clone(), 0);
        assert!(n3.is_some());
        assert_eq!(n3.unwrap().borrow().data, 0);

        assert!(t._find_node(t.root.clone(), 5).is_none());
    }

    #[test]
    fn find_lowest() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(2);
        t.add(1);
        t.add(3);
        t.add(0);
        let n = t._find_lowest_value(&t.root);
        assert_eq!(n, Some(0));
    }

    #[test]
    fn find_successor_simple() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(2);
        t.add(1);
        t.add(3);
        assert_eq!(t.find_successor(2), Some(3));
    }

    #[test]
    fn find_successor_unbalanced() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(1);
        t.add(2);
        t.add(3);
        t.add(4);
        t.add(5);
        t.add(6);
        assert_eq!(t.find_successor(5), Some(6));
    }

    #[test]
    fn find_successor_of_left_child() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(2);
        t.add(1);
        t.add(3);
        assert_eq!(t.find_successor(1), Some(2));
    }

    #[test]
    fn find_successor_of_right_child() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(2);
        t.add(1);
        t.add(3);
        assert_eq!(t.find_successor(3), None);
    }

    #[test]
    fn find_successor_of_right_child_with_left_null() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(2);
        t.add(3);
        assert_eq!(t.find_successor(3), None);
    }
}
