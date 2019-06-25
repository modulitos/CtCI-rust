// Validate BST: Implement a function to check if a binary tree is a
// binary search tree.

use cracking::{BinarySearchTree, Tree, TreeNode};

trait ValidateBST<T> {
    fn validate_bst(&self) -> bool;

    fn _is_tree_node_valid(&self, node: &Tree<T>) -> bool;
}

impl<T> ValidateBST<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn validate_bst(&self) -> bool {
        self._is_tree_node_valid(&self.root)
    }

    fn _is_tree_node_valid(&self, node: &Tree<T>) -> bool {
        // A BST is valid if its left and right subtrees are valid,
        // and if the value of the left child is less than the
        // current, and the value of the right child is more than the
        // current
        if let Some(n) = node {
            return match (&n.left, &n.right) {
                (Some(l), Some(r)) => {
                    self._is_tree_node_valid(&n.left)
                        && self._is_tree_node_valid(&n.right)
                        && l.data < n.data
                        && n.data < r.data
                }
                (None, Some(r)) => {
                    self._is_tree_node_valid(&n.right) && n.data < r.data
                }
                (Some(l), None) => self._is_tree_node_valid(&n.left) && l.data < n.data,
                (None, None) => true,
            }
        }
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

    #[test]
    fn simple_invalid_tree() {
        let mut root = TreeNode::<u32>::new_node(2);
        root.set_left(TreeNode::new_node(3));
        root.set_right(TreeNode::new_node(3));
        let t = BinarySearchTree::<u32>::new().with_root(root).create();
        assert_eq!(t.validate_bst(), false);
    }
}
