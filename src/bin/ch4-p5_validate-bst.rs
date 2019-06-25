// Validate BST: Implement a function to check if a binary tree is a
// binary search tree.

use cracking::{BinarySearchTree, Tree, TreeNode};

trait ValidateBST<T> {
    fn validate_bst(&self) -> bool;

    // fn validate_in_order(&self, node: &Tree<T>, prev: T) -> bool;
}

impl<T> ValidateBST<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn validate_bst(&self) -> bool {
        true
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_test_tree() {
        let mut tree = BinarySearchTree::<u32>::new();
        let mut root = TreeNode::new(2);
        let mut raw_root = root.take().unwrap();
        raw_root.left = TreeNode::new(1);
        raw_root.right = TreeNode::new(3);
        tree.root = root;
        assert_eq!(tree.validate_bst(), true);
    }
}
