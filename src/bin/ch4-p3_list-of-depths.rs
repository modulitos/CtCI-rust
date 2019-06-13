// List of Depths: Given a binary tree, design an algorithm which
// creates a linked list of all the nodes at each depth (e.g., if you
// have a tree with depth 0, you'll have 0 linked lists).

use cracking::BinarySearchTree;
use std::collections::LinkedList;

trait ListOfDepths<T> {
    fn get_lists(&self) -> Vec<LinkedList<T>>;
}

impl<T> ListOfDepths<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn get_lists(&self) -> Vec<LinkedList<T>> {
        vec![LinkedList::new()]
    }
}

mod tests {
    use super::*;

    #[test]
    fn get_lists_empty() {
        let bst = BinarySearchTree::<u32>::new();
        bst.get_lists();
    }
}
