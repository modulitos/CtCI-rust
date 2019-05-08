// Intersection: Given two (singly) linked lists, determine if the two
// lists intersect. Return the inter-secting node. Note that the
// intersection is defined based on reference, not value.

// That is, if the kth node of the first linked list is the exact same
// node (by reference) as the jth node of the second linked list, then
// they are intersecting.

extern crate cracking;
use cracking::{HashedNode, Node, NodeRef, SinglyLinkedList as LinkedList};

use std::collections::HashSet;

use std::cell::RefCell;
use std::rc::Rc;

pub trait LoopDetection<T> {
    fn detect_loop(&self) -> Option<NodeRef<T>>;
    fn detect_loop_with_set(&self) -> Option<NodeRef<T>>;
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
    fn detect_loop(&self) -> Option<NodeRef<T>> {
        let mut slow_iter = self.iter();
        let mut fast_iter = self.iter();
        while let (Some(slow), Some(_)) = (slow_iter.next(), fast_iter.next()) {
            // fast_iter moves 2 steps per iteration:
            match fast_iter.next() {
                None => return None, // no loop!
                Some(fast) => {
                    if Rc::ptr_eq(&slow, &fast) {
                        // loop is detected here - this is our
                        // collision point.
                        break;
                    }
                }
            }
        }
        // If the head is k steps from the start of the loop, then
        // when slow_iter reaches the start of the loop, fast_iter is
        // k steps ahead of slow_iter.

        // If `K = k % loop_size`, then fast_iter is K steps ahead of
        // slow iter, and loop_size - K steps behind slow iter. So
        // slow_iter will move loop_size - K steps once they collide,
        // making the collision point K steps behind the start of he
        // loop. This is the same as being k steps behind the start of
        // the loop!

        // Thus, we know that the collision point is the same number
        // of nodes away from the loop start as the head, so let's set
        // a new pointer to the head, and move along both iterators
        // until they collide again at the start of the loop.
        slow_iter = self.iter();
        while let (Some(slow), Some(fast)) = (slow_iter.next(), fast_iter.next()) {
            if Rc::ptr_eq(&slow, &fast) {
                // This is the starting node of our loop.
                return Some(fast);
            }
        }
        None
    }

    fn detect_loop_with_set(&self) -> Option<NodeRef<T>> {
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
        assert_eq!(list.detect_loop_with_set(), None);
    }

    #[test]
    fn detect_loop_none() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.detect_loop(), None);
        assert_eq!(list.detect_loop_with_set(), None);
    }

    #[test]
    fn detect_loop_none_short() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(1);
        assert_eq!(list.detect_loop(), None);
        assert_eq!(list.detect_loop_with_set(), None);
    }

    #[test]
    fn detect_loop_even() {
        let node = Rc::new(RefCell::new(Node {
            data: 9,
            next: None,
        }));
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append_node(node.clone());
        list.append(1);
        list.append(2);
        list.append(3);
        list.append_node(node.clone());
        assert!(Rc::ptr_eq(&list.detect_loop().unwrap(), &node));
        assert!(Rc::ptr_eq(&list.detect_loop_with_set().unwrap(), &node));
    }

    #[test]
    fn detect_loop_odd() {
        let node = Rc::new(RefCell::new(Node {
            data: 9,
            next: None,
        }));
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append_node(node.clone());
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append_node(node.clone());
        assert!(Rc::ptr_eq(&list.detect_loop().unwrap(), &node));
    }

    #[test]
    fn detect_loop_with_tail() {
        let node = Rc::new(RefCell::new(Node {
            data: 9,
            next: None,
        }));
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append_node(node.clone());
        list.append(4);
        list.append(5);
        list.append(6);
        list.append(7);
        list.append(8);
        list.append_node(node.clone());
        assert!(Rc::ptr_eq(&list.detect_loop().unwrap(), &node));
    }
}
