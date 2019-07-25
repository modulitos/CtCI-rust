// Check Subtree: Tl and T2 are two very large binary trees, with Tl
// much bigger than T2. Create an algorithm to determine if T2 is a
// subtree of Tl. A tree T2 is a subtree ofTi if there exists a node n
// in Ti such that the subtree of n is identical to T2. That is, if
// you cut off the tree at node n, the two trees would be identical.

// Clarifying questions:
// Do we have a preference for memory vs runtime?
// Will the data in T1 or T2 be very large? This can affect whether we want to tradeoff runtime performance for memory performance.

use cracking::{BTree as Tree, BTreeNode as Node, BinaryTree};

// Uses a string comparison approach - serialize the trees into
// strings, then check whether the substree's string is contained
// within the tree's string.
// space: O(n + m)
// time: O(n + m)
fn is_subtree_string<T>(tree: &Tree<T>, subtree: &Tree<T>) -> bool
where
    T: std::string::ToString,
{
    tree_to_string(tree).contains(&tree_to_string(subtree))
}

// Performes an in-order traversal, and prints the results into a string
fn tree_to_string<T>(tree: &Tree<T>) -> String
where
    T: std::string::ToString,
{
    let mut results: String = "".to_owned();
    if let Some(node) = tree {
        results.push_str(&format!("left({})", &tree_to_string(&node.left)));
        results.push_str(&format!(" {} ", &node.data.to_string()));
        results.push_str(&format!("right({})", &tree_to_string(&node.right)));
    } else {
        results.push_str(" NULL ");
    }
    results
}

// A more space-efficient approach, instead of serializing the entire tree:
// space: O(1)
// time: O(n*m) - although it's much less than that
fn is_subtree<T>(tree: &Tree<T>, subtree: &Tree<T>) -> bool
where
    T: std::cmp::PartialEq + std::fmt::Debug,
{
    match (&tree, &subtree) {
        (Some(t), Some(s)) => {
            if t.data == s.data && is_subtree(&t.left, &s.left) && is_subtree(&t.right, &s.right) {
                // if the nodes and their children are equal:
                return true;
            } else {
                // if the nodes or their children are not equal,
                // continue traversing through the main tree:
                is_subtree(&t.left, subtree) || is_subtree(&t.right, subtree)
            }
        }
        // base cases
        (None, None) => true,
        _ => false,
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_tree_to_string_empty() {
        let bt = BinaryTree::<u32>::new(None);
        assert_eq!(tree_to_string(&bt.root), " NULL ");
    }

    #[test]
    fn test_tree_to_string_simple() {
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, None), None),
        );
        let bt = BinaryTree::<u32>::new(root.clone());
        assert_eq!(
            tree_to_string(&bt.root),
            "left(left( NULL ) 2 right( NULL )) 1 right(left(left( NULL ) 1 right( NULL )) 1 right( NULL ))"
        );
    }

    #[test]
    fn test_tree_in_subtree_strings() {
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, None), None),
        );
        let bt = BinaryTree::<u32>::new(root.clone());

        let bt_sub = BinaryTree::<u32>::new(Node::new(1, Node::new(1, None, None), None));
        assert_eq!(is_subtree_string(&bt.root, &bt_sub.root), true);
        let bt_sub2 = BinaryTree::<u32>::new(Node::new(
            1,
            Node::new(1, None, None),
            Node::new(1, None, None),
        ));
        assert_eq!(is_subtree_string(&bt.root, &bt_sub2.root), false);
    }

    #[test]
    fn test_tree_in_subtree_alt() {
        let root = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(1, Node::new(1, None, None), None),
        );
        let bt = BinaryTree::<u32>::new(root.clone());

        let bt_sub = BinaryTree::<u32>::new(Node::new(1, Node::new(1, None, None), None));
        assert_eq!(is_subtree(&bt.root, &bt_sub.root), true);
        let bt_sub2 = BinaryTree::<u32>::new(Node::new(
            1,
            Node::new(1, None, None),
            Node::new(1, None, None),
        ));
        assert_eq!(is_subtree(&bt.root, &bt_sub2.root), false);
    }
}
