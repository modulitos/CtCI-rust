// Check Balanced: Implement a function to check if a binary tree is
// balanced. For the purposes of this question, a balanced tree is
// defined to be a tree such that the heights of the two subtrees of
// any node never differ by more than one.

use cracking::{BinarySearchTree, Tree};
use std::cmp;

trait CheckBalanced<T> {
    fn check_balanced(&self) -> bool;
    fn check_node_balanced(&self, node: &Tree<T>) -> Option<i32>;
}

impl<T> CheckBalanced<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn check_balanced(&self) -> bool {
        self.check_node_balanced(&self.root).is_some()
    }

    fn check_node_balanced(&self, node: &Tree<T>) -> Option<i32> {
        if let Some(n) = node {
            let left = self.check_node_balanced(&n.left);
            let right = self.check_node_balanced(&n.right);
            if let (Some(l), Some(r)) = (left, right) {
                if (l - r).abs() < 2 {
                    Some(1 + cmp::max(l, r))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            Some(0)
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn check_balanced_empty() {
        let bst = BinarySearchTree::<u32>::new();
        assert_eq!(bst.check_balanced(), true);
    }

    #[test]
    fn check_balanced_root_only() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        assert_eq!(bst.check_balanced(), true);
    }

    #[test]
    fn check_balanced_simple() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        bst.add(1);
        bst.add(3);
        assert_eq!(bst.check_balanced(), true);
    }

    #[test]
    fn check_unbalanced_simple() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(1);
        bst.add(2);
        bst.add(3);
        assert_eq!(bst.check_balanced(), false);
    }

    #[test]
    fn check_balanced_5() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        bst.add(1);
        bst.add(0);
        bst.add(3);
        bst.add(4);
        assert_eq!(bst.check_balanced(), true);
    }

    #[test]
    fn check_unbalanced_6() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        bst.add(1);
        bst.add(0);
        bst.add(3);
        bst.add(4);
        bst.add(5);
        assert_eq!(bst.check_balanced(), false);
    }
}
