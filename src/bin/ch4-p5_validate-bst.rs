// Validate BST: Implement a function to check if a binary tree is a
// binary search tree.

use cracking::{BinarySearchTree, TreeNode};

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
        let mut root = TreeNode::<u32>::new_node(2);
        root.set_left(TreeNode::new_node(1));
        root.set_right(TreeNode::new_node(3));
        let t = BinarySearchTree::<u32>::new().with_root(root).create();
        assert_eq!(t.validate_bst(), true);
    }

    // #[test]
    // fn simple_invalid_tree() {
    //     let mut root = TreeNode::<u32>::new_node(2);
    //     root.set_left(TreeNode::new_node(3));
    //     root.set_right(TreeNode::new_node(3));
    //     let t = BinarySearchTree::<u32>::new().with_root(root).create();
    //     assert_eq!(t.validate_bst(), true);
    // }
}
