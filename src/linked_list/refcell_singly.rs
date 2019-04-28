use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::ptr;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<NodeRef<T>>,
}

pub struct Node<T> {
    pub data: T,
    pub next: Option<NodeRef<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ data: {:?}, next: {:?} }}", self.data, self.next)
    }
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&mut self, new_value: T) {
        let new_node = Some(Rc::new(RefCell::new(Node {
            data: new_value,
            next: self.head.take(),
        })));
        self.head = new_node.clone();
    }

    pub fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        }
    }

    pub fn append_node(&mut self, node: NodeRef<T>) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(node.clone());
        } else {
            self.head = Some(node.clone());
        }
    }

    pub fn tail(&self) -> Option<NodeRef<T>> {
        for node in self.iter() {
            if let None = node.clone().borrow().next {
                return Some(node);
            }
        }
        None
    }

    /// Warning: this will not check that the provided node belongs to the current list.
    fn _unlink_node(&mut self, node_to_remove: Option<NodeRef<T>>) {
        let node_to_remove = node_to_remove.unwrap();

        for node in self.iter() {
            let mut borrowed_node = node.borrow_mut();
            if let Some(next) = borrowed_node.next.clone() {
                if Rc::ptr_eq(&next, &node_to_remove) {
                    borrowed_node.next = node_to_remove.borrow_mut().next.take();
                    break;
                }
            } else if Rc::ptr_eq(&node, &node_to_remove) {
                // handle case when node_to_remove is the only element
                // in the list
                self.head = None;
            }
        }
    }

    // /// Warning: this will not check that the provided node belongs to the current list.
    // pub fn unlink_node(&mut self, node_to_remove: Option<NodeRef<T>>) {
    //     let node_to_remove = node_to_remove.unwrap();

    //     match node_to_remove.borrow().prev.clone() {
    //         Some(prev) => prev.borrow_mut().next = node_to_remove.borrow().next.clone(),
    //         // if we removed the head, assign new head:
    //         None => self.head = node_to_remove.borrow().next.clone(),
    //     };

    //     match node_to_remove.borrow().next.clone() {
    //         Some(next) => (),
    //         // if we remove the tail, assign new tail:
    //         None => ()
    //     };
    // }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Iter<T> {
    next: Option<NodeRef<T>>,
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.clone() {
            // Set the new self.next:
            // if Rc::ptr_eq(self.next.as_ref().unwrap(), self.last.as_ref().unwrap()) {

            if let Some(new_next) = next.borrow().next.clone() {
                self.next = Some(new_next);
            } else {
                self.next = None;
            }

            // self.next = next.borrow().next.clone();
            return Some(next);
        } else {
            None
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

impl<T: PartialEq + fmt::Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if ptr::eq(self, other) {
            // For loop detection - if the nodes share the same
            // reference, they are equal.
            return true;
        }
        self.data == other.data && self.next == other.next
    }

    fn ne(&self, other: &Self) -> bool {
        if !ptr::eq(self, other) {
            return true;
        }
        self.data != other.data && self.next == other.next
    }
}

impl<T: PartialEq + std::fmt::Debug> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
    }

    fn ne(&self, other: &Self) -> bool {
        // println!(
        //     "comparing two lists: self.head: {:?}, other.head: {:?}",
        //     self.head, other.head
        // );
        self.head != other.head
    }
}

impl<T: Eq + std::fmt::Debug> Eq for LinkedList<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);
        list2.append(3);

        assert_eq!(list, list2);
        list2 = LinkedList::new();
        list2.append(3);
        assert_ne!(list, list2);
        list = LinkedList::new();
        list.append(3);
        assert_eq!(list, list2);
    }

    #[test]
    fn prepend_and_append() {
        let mut list = LinkedList::new();
        list.prepend(2);
        list.prepend(1);
        list.append(3);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);
        list2.append(3);

        assert_eq!(list, list2);
        list2.prepend(1);
        assert_ne!(list, list2);
        list.prepend(1);
        assert_eq!(list, list2);
    }

    #[test]
    fn eq_append_node() {
        let shared_node = Rc::new(RefCell::new(Node {
            data: 1,
            next: None,
        }));
        let mut list1 = LinkedList::new();
        list1.append_node(shared_node.clone());

        let mut list2 = LinkedList::new();
        list2.append(1);

        assert_eq!(list1, list2);

        let mut list3 = LinkedList::new();
        list3.append_node(shared_node);
        list3.append(2);

        let mut list4 = LinkedList::new();
        list4.append(1);
        list4.append(2);

        assert_eq!(list3, list4);

        // These are no longer equal! Because a new node has been added to the shared_node:
        assert_ne!(list1, list2);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);

        assert_eq!(list1, list2);
    }

    #[test]
    fn append_many_nodes() {
        let node = Rc::new(RefCell::new(Node {
            data: 3,
            next: None,
        }));
        let mut list1 = LinkedList::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);

        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(2);
        list2.append_node(node);

        assert_eq!(list1, list2);
        list1.append(4);
        assert_ne!(list1, list2);
        list2.append(4);
        assert_eq!(list1, list2);
        let node = Rc::new(RefCell::new(Node {
            data: 9,
            next: None,
        }));
        list1.append_node(node.clone());
        list2.append_node(node);
        list2.append(3);
        assert_eq!(list1, list2);
    }

    #[test]
    fn eq_with_cycle() {
        let first_node = Rc::new(RefCell::new(Node {
            data: 1,
            next: None,
        }));
        let mut list = LinkedList::new();
        list.append_node(first_node.clone());
        list.append(2);
        list.append(3);
        list.append_node(first_node.clone());

        let mut list2 = LinkedList::new();
        list2.append(1);

        assert_ne!(list, list2);

        let mut list2 = LinkedList::new();
        list2.append_node(first_node);

        assert_eq!(list, list2);
    }
}
