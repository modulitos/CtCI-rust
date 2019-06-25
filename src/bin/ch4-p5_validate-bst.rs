// Validate BST: Implement a function to check if a binary tree is a
// binary search tree.

// clarifying questions:
// - how to handle duplicate values?
// - does the tree only contain numbers?

use cracking::{BinarySearchTree, Tree, TreeNode};

trait ValidateBST<T> {
    fn validate_bst(&self) -> bool;

    fn _is_tree_node_valid(&self, node: &Tree<T>, min: Option<T>, max: Option<T>) -> bool;
}

impl<T> ValidateBST<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn validate_bst(&self) -> bool {
        self._is_tree_node_valid(&self.root, None, None)
    }

    fn _is_tree_node_valid(&self, node: &Tree<T>, min: Option<T>, max: Option<T>) -> bool {
        // A BST is valid if its left and right subtrees are valid,
        // meaning that if the max value of the left subtree is less
        // than or equal to the current value, and the min value of
        // the left subtree is greater than the current

        // Returns the min/max values of this node's subtree, or None
        // if this node's subtree is invalid.

        if let Some(n) = node {
            // check whether the left subtree is valid:
            if let Some(mn) = min.clone() {
                if n.data <= mn {
                    return false;
                }
            }
            if !self._is_tree_node_valid(&n.left, min, Some(n.data.clone())) {
                return false;
            }

            // check whether the right subtree is valid:
            if let Some(mx) = max.clone() {
                if n.data > mx {
                    return false;
                }
            }
            if !self._is_tree_node_valid(&n.right, Some(n.data.clone()), max) {
                return false;
            }

            true
        } else {
            // We are at a leaf node:
            true
        }
    }
}

// A simpler version, but limited to a u32:
trait ValidateBSTU32 {
    fn validate_bst_u32(&self) -> bool;

    fn _is_tree_node_valid_u32(&self, node: &Tree<u32>, min: u32, max: u32) -> bool;
}

impl ValidateBSTU32 for BinarySearchTree<u32> {
    fn validate_bst_u32(&self) -> bool {
        self._is_tree_node_valid_u32(&self.root, u32::min_value(), u32::max_value())
    }

    fn _is_tree_node_valid_u32(&self, node: &Tree<u32>, min: u32, max: u32) -> bool {
        if let Some(n) = node {
            if n.data < min || max <= n.data {
                return false;
            }

            return self._is_tree_node_valid_u32(&n.left, min, n.data + 1)
                && self._is_tree_node_valid_u32(&n.right, n.data, max);
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
        assert_eq!(t.validate_bst_u32(), true);
    }

    #[test]
    fn simple_invalid_tree() {
        let mut root = TreeNode::<u32>::new_node(2);
        root.set_left(TreeNode::new_node(3));
        root.set_right(TreeNode::new_node(3));
        let t = BinarySearchTree::<u32>::new().with_root(root).create();
        assert_eq!(t.validate_bst(), false);
        assert_eq!(t.validate_bst_u32(), false);
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
        assert_eq!(t.validate_bst_u32(), true);
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
        assert_eq!(t.validate_bst_u32(), true);
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
        assert_eq!(t.validate_bst_u32(), true);
    }

    #[test]
    fn invalid_complex_tree_right_less_than_left() {
        let mut root = TreeNode::<u32>::new_node(3);
        let mut root_right = TreeNode::<u32>::new_node(9);
        root_right.set_right(TreeNode::new_node(7)); // out of order!
        let mut root_left = TreeNode::<u32>::new_node(2);
        root_left.set_left(TreeNode::new_node(1));
        root_left.set_right(TreeNode::new_node(3));
        root.set_left(root_left);
        root.set_right(root_right);
        let t = BinarySearchTree::<u32>::new().with_root(root).create();
        assert_eq!(t.validate_bst(), false);
        assert_eq!(t.validate_bst_u32(), false);
    }

    #[test]
    fn invalid_complex_tree_left_greater_than_grandparent() {
        let mut root = TreeNode::<u32>::new_node(3);
        let mut root_right = TreeNode::<u32>::new_node(9);
        root_right.set_right(TreeNode::new_node(10));
        let mut root_left = TreeNode::<u32>::new_node(2);
        root_left.set_left(TreeNode::new_node(1));
        root_left.set_right(TreeNode::new_node(4)); // this is greater than root!
        root.set_left(root_left);
        root.set_right(root_right);
        let t = BinarySearchTree::<u32>::new().with_root(root).create();
        assert_eq!(t.validate_bst(), false);
        assert_eq!(t.validate_bst_u32(), false);
    }
}
