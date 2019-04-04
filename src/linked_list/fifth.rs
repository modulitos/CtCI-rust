// largely based off of second.rs implementation
// https://rust-unofficial.github.io/too-many-lists/fifth-final.html
use std::collections::HashSet;
use std::ptr;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER - raw pointer
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
where
    T: std::hash::Hash + std::cmp::Eq + std::marker::Copy,
{
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        // Put the box in the right place, and then grab a reference to its Node
        if !self.tail.is_null() {
            // If the old tail existed, update it to point to the new tail
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // Otherwise, update the head to point to it
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }

    // pub fn remove_dups(&mut self) {
    //     let mut set: HashSet<T> = HashSet::new();
    //     let mut curr: Link<T> = self.head.take();
    //     let mut prev: Link<T> = None;
    //     while curr.is_some() {
    //         if set.contains(&curr.as_ref().unwrap().elem) {
    //             let raw_prev: *mut _ = &mut *(prev.unwrap());
    //             let raw_curr: *mut _ = &mut *(curr.unwrap());

    //             unsafe {
    //                 let next = &(*raw_curr).next;
    //                 if next.is_some() {
    //                     // COMPILE ERROR: cannot move out of dereference of raw pointer:
    //                     (*raw_prev).next = (*raw_curr).next;
    //                     // COMPILE ERROR: cannot move out of dereference of raw pointer:
    //                     curr = (*raw_curr).next;
    //                     // COMPILE ERROR: cannot move out of dereference of raw pointer:
    //                     prev = Some(Box::new(*raw_curr));
    //                     // prev = curr;
    //                 } else {
    //                     // If the tail is being removed, then set tail
    //                     // pointer to be prev:
    //                     self.tail = raw_prev;
    //                     (*raw_prev).next = None;
    //                     // COMPILE ERROR: cannot move out of dereference of raw pointer:
    //                     prev = Some(Box::new(*raw_curr));
    //                     curr = None;
    //                 }
    //             }
    //         } else {
    //             set.insert(curr.as_ref().unwrap().elem);
    //             curr = curr.unwrap().next;
    //             prev = curr.take();
    //         }
    //     }
    // }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem && self.next == other.next
    }

    fn ne(&self, other: &Self) -> bool {
        self.elem != other.elem || self.next != other.next
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
    }

    fn ne(&self, other: &Self) -> bool {
        self.head != other.head
    }
}

impl<T: Eq> Eq for List<T> {}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn eq() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut list2 = List::new();
        list2.push(1);
        list2.push(2);
        list2.push(3);

        assert_eq!(list, list2);
        list2.pop();
        assert_ne!(list, list2);
        list.pop();
        assert_eq!(list, list2);
    }

    // #[test]
    // fn remove_dups() {
    //     let mut list = List::new();
    //     list.push(1);
    //     list.push(2);
    //     list.push(2);
    //     list.push(3);
    //     list.remove_dups();
    //     let mut list2 = List::new();
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);
    //     assert_eq!(list, list2);
    // }
}
