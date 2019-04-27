// Intersection: Given two (singly) linked lists, determine if the two
// lists intersect. Return the inter-secting node. Note that the
// intersection is defined based on reference, not value.

// That is, if the kth node of the first linked list is the exact same
// node (by reference) as the jth node of the second linked list, then
// they are intersecting.


extern crate cracking;
use cracking::{SinglyLinkedList as LinkedList, Node};

use std::rc::Rc;
use std::cell::RefCell;

pub trait CheckIntersection<T> {
    fn is_intersection(&self, other: &LinkedList<T>) -> bool;
}

impl<T> CheckIntersection<T> for LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    fn is_intersection(&self, other: &LinkedList<T>) -> bool {
        for node in self.iter() {
            for node2 in other.iter() {
                if Rc::ptr_eq(&node, &node2) {
                    return true;
                }
            }
        }
        false
    }
}


mod test {
    use super::*;

    #[test]
    fn check_intersection_node() {
        let mut list1 = LinkedList::new();
        list1.append(3);
        list1.append(2);
        list1.append(1);
        let mut list2 = LinkedList::new();
        list2.append(3);
        list2.append(2);
        list2.append(1);
        assert_ne!(list1.is_intersection(&list2), true);
    }

    #[test]
    fn check_no_intersection() {
        let mut list1 = LinkedList::new();
        list1.append(3);
        let mut list2 = LinkedList::new();
        list2.append(3);
        assert_eq!(list1.is_intersection(&list2), false);
    }

    #[test]
    fn check_single_intersection() {
        let common_node = Rc::new(RefCell::new(Node{ data: 9, next: None}));
        let mut list1 = LinkedList::new();
        list1.append_node(common_node.clone());
        let mut list2 = LinkedList::new();
        list2.append_node(common_node);
        assert_eq!(list1.is_intersection(&list2), true);
    }

    #[test]
    fn check_intersection() {
        let common_node = Rc::new(RefCell::new(Node{ data: 9, next: None}));
        let mut list1 = LinkedList::new();
        list1.append(3);
        list1.append(2);
        list1.append_node(common_node.clone());
        list1.append(1);
        let mut list2 = LinkedList::new();
        list1.append(3);
        list1.append(2);
        assert_eq!(list1.is_intersection(&list2), false);
        list2.append_node(common_node);
        assert_eq!(list1.is_intersection(&list2), true);
        list1.append(2);
        list2.append(2);
        assert_eq!(list1.is_intersection(&list2), true);
    }
}
