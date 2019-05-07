// Intersection: Given two (singly) linked lists, determine if the two
// lists intersect. Return the inter-secting node. Note that the
// intersection is defined based on reference, not value.

// That is, if the kth node of the first linked list is the exact same
// node (by reference) as the jth node of the second linked list, then
// they are intersecting.

extern crate cracking;
use cracking::{SinglyLinkedList as LinkedList, Node, NodeRef, HashedNode};

use std::collections::HashSet;

use std::rc::Rc;
use std::cell::RefCell;

pub trait LoopDetection<T> {
    fn detect_loop(&self) -> Option<NodeRef<T>>;
}

impl<T> LoopDetection<T> for LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    fn detect_loop(&self) -> Option<NodeRef<T>>{
        let mut visited_nodes: HashSet<HashedNode<T>> = HashSet::new();
        for node in self.iter() {
            if visited_nodes.contains(&HashedNode::from_node(node.clone())) {
                return Some(node);
            } else {
                visited_nodes.insert(HashedNode::from_node(node));
            }
        }
        None
    }
}


mod test {
    use super::*;

    #[test]
    fn detect_loop_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.detect_loop(), None);
    }

    #[test]
    fn detect_loop_none() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.detect_loop(), None);
    }

    #[test]
    fn detect_loop_none_short() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(1);
        assert_eq!(list.detect_loop(), None);
    }

    #[test]
    fn detect_loop_even() {
        let node = Rc::new(RefCell::new(Node{ data: 9, next: None}));
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append_node(node.clone());
        list.append(1);
        list.append(2);
        list.append(3);
        list.append_node(node.clone());
        assert!(Rc::ptr_eq(&list.detect_loop().unwrap(), &node));
    }
}
