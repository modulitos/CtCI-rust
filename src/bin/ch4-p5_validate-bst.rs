// Validate BST: Implement a function to check if a binary tree is a
// binary search tree.

// clarifying questions: how to handle duplicate values?

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
                        && l.data <= n.data
                        && n.data < r.data
                }
                (None, Some(r)) => self._is_tree_node_valid(&n.right) && n.data < r.data,
                (Some(l), None) => self._is_tree_node_valid(&n.left) && l.data <= n.data,
                (None, None) => true,
            };
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

    #[test]
    fn valid_tree_random() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(5);
        t.add(1);
        t.add(2);
        t.add(3);
        t.add(4);
        t.add(9);
        t.add(7);
        assert_eq!(t.validate_bst(), true);
    }

    #[test]
    fn valid_tree_unbalanced() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(1);
        t.add(2);
        t.add(3);
        t.add(4);
        t.add(5);
        t.add(6);
        t.add(7);
        assert_eq!(t.validate_bst(), true);
    }

    #[test]
    fn valid_tree_balanced() {
        let mut t = BinarySearchTree::<u32>::new();
        t.add(5);
        // root.left
        t.add(2);
        t.add(2); // duplicate value!
        t.add(0);
        t.add(3);
        t.add(4);
        // root.right
        t.add(9);
        t.add(7);
        t.add(6);
        t.add(8);
        t.add(13);
        t.add(11);
        t.add(12);
        t.add(10);
        t.add(15);
        t.add(16);
        t.add(14);
        assert_eq!(t.validate_bst(), true);
    }

    #[test]
    fn invalid_complex_tree() {
        let mut root = TreeNode::<u32>::new_node(3);
        let mut root_right = TreeNode::<u32>::new_node(9);
        root_right.set_right(TreeNode::new_node(7));  // out of order!
        let mut root_left = TreeNode::<u32>::new_node(2);
        root_left.set_left(TreeNode::new_node(1));
        root_left.set_right(TreeNode::new_node(3));
        root.set_left(root_left);
        root.set_right(root_right);
        let t = BinarySearchTree::<u32>::new().with_root(root).create();
        assert_eq!(t.validate_bst(), false);
    }
}
