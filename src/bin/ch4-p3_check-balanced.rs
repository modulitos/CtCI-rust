// Check Balanced: Implement a function to check if a binary tree is
// balanced. For the purposes of this question, a balanced tree is
// defined to be a tree such that the heights of the two subtrees of
// any node never differ by more than one.

use cracking::{BinarySearchTree, Tree};
use std::num;

trait CheckBalanced<T> {
    fn check_balanced(&self) -> bool;
    fn get_node_height(&self, node: &Tree<T>) -> i32;
}

impl<T> CheckBalanced<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn check_balanced(&self) -> bool {
        if let Some(root) = &self.root {
            (self.get_node_height(&root.left) - self.get_node_height(&root.right)).abs() < 2
        } else {
            true
        }
    }
    fn get_node_height(&self, node: &Tree<T>) -> i32 {
        0
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

    // #[test]
    // fn check_unbalanced_simple() {
    //     let mut bst = BinarySearchTree::<u32>::new();
    //     bst.add(1);
    //     bst.add(2);
    //     bst.add(3);
    //     assert_eq!(bst.check_balanced(), true);
    // }

}
