// First Common Ancestor: Design an algorithm and write code to find_common
// the first common ancestor of two nodes in a binary tree. Avoid
// storing additional nodes in a data structure. NOTE: This is not
// necessarily a binary search tree.

use cracking::{RCBTree as Tree, RCBTreeNode as Node, RCBinaryTree as BinaryTree};
use std::rc::Rc;

trait FirstCommonAncestor<T> {
    fn find_common(&self, n1: Tree<T>, n2: Tree<T>) -> Tree<T>;
    fn _get_depth(&self, node: Tree<T>) -> i32;
    fn find_common_alt(&self, n1: &Tree<T>, n2: &Tree<T>) -> Tree<T>;
    fn _is_node_in_tree(&self, node: &Tree<T>, tree: &Tree<T>) -> bool;
    fn find_common_alt_rec(&self, n1: &Tree<T>, n2: &Tree<T>, node: &Tree<T>) -> Tree<T>;
}

impl<T> FirstCommonAncestor<T> for BinaryTree<T> {
    // runtime is O(d) where d is the depth of the deepest node.
    fn find_common(&self, n1: Tree<T>, n2: Tree<T>) -> Tree<T> {
        let n1_depth = self._get_depth(n1.clone());
        let n2_depth = self._get_depth(n2.clone());
        let diff = n1_depth - n2_depth;
        let abs_diff = diff.abs();
        let (mut lower_node, mut higher_node) = if diff > 0 { (n1, n2) } else { (n2, n1) };
        for _ in 0..abs_diff {
            // Move the lower node up until it's at the same level
            if let Some(node) = lower_node {
                lower_node = node.borrow().parent.clone();
            }
        }
        while let (Some(lower), Some(higher)) = (lower_node, higher_node) {
            if Rc::ptr_eq(&lower, &higher) {
                return Some(lower);
            } else {
                lower_node = lower.borrow().parent.clone();
                higher_node = higher.borrow().parent.clone();
            }
        }
        panic!("the nodes don't have a common ancestor!");
    }

    fn _get_depth(&self, mut node: Tree<T>) -> i32 {
        let mut depth = 0;
        while let Some(n) = node {
            node = n.borrow().parent.clone();
            depth += 1;
        }
        depth
    }

    // This implementation is best when the nodes don't have links to parents:
    // TODO: This can be optimized. Current runtime is O(n) where n is number of nodes in the tree.
    fn find_common_alt(&self, n1: &Tree<T>, n2: &Tree<T>) -> Tree<T> {
        // Find the first node where n1 and n2 is split across the left and right nodes:
        self.find_common_alt_rec(n1, n2, &self.root)
    }

    fn find_common_alt_rec(&self, n1: &Tree<T>, n2: &Tree<T>, node: &Tree<T>) -> Tree<T> {
        if let Some(n) = node {
            let is_n1_on_left = self._is_node_in_tree(n1, &n.borrow().left);
            let is_n2_on_left = self._is_node_in_tree(n2, &n.borrow().left);
            match (
                is_n1_on_left,
                is_n2_on_left,
            ) {
                (true, true) => self.find_common_alt_rec(n1, n2, &n.borrow().left),
                (false, false) => self.find_common_alt_rec(n1, n2, &n.borrow().right),
                (false, true) => {
                    if self._is_node_in_tree(n1, &n.borrow().right) {
                        node.clone()
                    } else {
                        // n1 is not on the left nor the right!
                        panic!("the nodes don't have a common ancestor!")
                    }
                }
                (true, false) => {
                    if self._is_node_in_tree(n2, &n.borrow().right) {
                        node.clone()
                    } else {
                        // n2 is not on the left nor the right!
                        panic!("the nodes don't have a common ancestor!")
                    }
                }             }
        } else {
            None
        }
    }

    fn _is_node_in_tree(&self, node: &Tree<T>, tree: &Tree<T>) -> bool {
        if let (Some(n), Some(haystack)) = (node, tree) {
            if Rc::ptr_eq(n, haystack) {
                true
            } else {
                self._is_node_in_tree(node, &haystack.borrow().left)
                    || self._is_node_in_tree(node, &haystack.borrow().right)
            }
        } else {
            false
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_depth_simple() {
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, None), None),
        );
        let bt = BinaryTree::<u32>::new(root.clone());
        assert_eq!(bt._get_depth(root), 1);
    }

    #[test]
    fn test_get_depth() {
        let node = Node::new(1, None, Node::new(1, None, None));
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, node.clone()), None),
        );
        let bt = BinaryTree::<u32>::new(root);
        assert_eq!(bt._get_depth(node), 4);
    }

    #[test]
    fn test_search_node_simple() {
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, None), None),
        );
        let bt = BinaryTree::<u32>::new(root.clone());
        assert_eq!(
            bt._is_node_in_tree(&root, &root),
            true
        );
    }

    #[test]
    fn test_search_node() {
        let node = Node::new(1, None, Node::new(1, None, None));
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, node.clone()), None),
        );
        let bt = BinaryTree::<u32>::new(root.clone());
        let nonexistent = Node::new(1, None, Node::new(1, None, None));
        assert_eq!(bt._is_node_in_tree(&node, &root), true);
        assert_eq!(bt._is_node_in_tree(&nonexistent, &root), false);
    }

    #[test]
    fn test_simple_bt() {
        let n1 = Node::new(1, None, None);
        let n2 = Node::new(2, None, None);
        let node = Node::new(1, n1.clone(), n2.clone());
        let bt = BinaryTree::<u32>::new(node.clone());
        assert!(Rc::ptr_eq(
            bt.find_common(n1, n2).as_ref().unwrap(),
            &node.as_ref().unwrap(),
        ));
    }

    #[test]
    fn test_simple_bt_alt() {
        let n1 = Node::new(1, None, None);
        let n2 = Node::new(2, None, None);
        let node = Node::new(1, n1.clone(), n2.clone());
        let bt = BinaryTree::<u32>::new(node.clone());
        assert!(Rc::ptr_eq(
            bt.find_common_alt(&n1, &n2).as_ref().unwrap(),
            &node.as_ref().unwrap(),
        ));
    }

    #[test]
    fn test_complex_bt() {
        let n1 = Node::new(1, Node::new(1, None, None), None);
        let n2 = Node::new(2, None, Node::new(1, None, None));
        let parent = Node::new(
            2,
            Node::new(1, Node::new(1, n2.clone(), None), None),
            Node::new(1, None, Node::new(1, None, n1.clone())),
        );
        let node = Node::new(1, parent.clone(), None);
        let bt = BinaryTree::<u32>::new(node.clone());
        assert!(Rc::ptr_eq(
            bt.find_common(n1, n2).as_ref().unwrap(),
            &parent.as_ref().unwrap(),
        ));
    }

    #[test]
    fn test_complex_bt_alt() {
        let n1 = Node::new(1, Node::new(1, None, None), None);
        let n2 = Node::new(2, None, Node::new(1, None, None));
        let parent = Node::new(
            2,
            Node::new(1, Node::new(1, n2.clone(), None), None),
            Node::new(1, None, Node::new(1, None, n1.clone())),
        );
        let node = Node::new(1, parent.clone(), None);
        let bt = BinaryTree::<u32>::new(node.clone());
        assert!(Rc::ptr_eq(
            bt.find_common_alt(&n1, &n2).as_ref().unwrap(),
            &parent.as_ref().unwrap(),
        ));
    }
    #[test]
    #[should_panic(expected = "the nodes don't have a common ancestor!")]
    fn test_fail_bt() {
        let n1 = Node::new(1, Node::new(1, None, None), None);
        let n2 = Node::new(2, None, Node::new(1, None, None));
        let parent = Node::new(
            2,
            Node::new(1, Node::new(1, n2.clone(), None), None),
            Node::new(1, None, Node::new(1, None, None)),
        );
        let node = Node::new(1, parent.clone(), None);
        let bt = BinaryTree::<u32>::new(node.clone());
        bt.find_common(n1, n2).as_ref().unwrap();
    }

    #[test]
    #[should_panic(expected = "the nodes don't have a common ancestor!")]
    fn test_fail_bt_alt() {
        let n1 = Node::new(1, Node::new(1, None, None), None);
        let n2 = Node::new(2, None, Node::new(1, None, None));
        let parent = Node::new(
            2,
            Node::new(1, Node::new(1, n2.clone(), None), None),
            Node::new(1, None, Node::new(1, None, None)),
        );
        let node = Node::new(1, parent.clone(), None);
        let bt = BinaryTree::<u32>::new(node.clone());
        bt.find_common_alt(&n1, &n2).as_ref().unwrap();
    }
}
