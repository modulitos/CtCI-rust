use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<NodeRef<T>>,
    pub tail: Option<NodeRef<T>>,
}

pub struct Node<T> {
    pub data: T,
    pub next: Option<NodeRef<T>>,
    pub prev: Option<NodeRef<T>>,
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
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn prepend(&mut self, new_value: T) {
        let new_node = Some(Rc::new(RefCell::new(Node {
            data: new_value,
            next: self.head.take(),
            prev: None,
        })));
        self.head = new_node.clone();
        if let None = self.tail {
            self.tail = new_node;
        }
    }

    pub fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail() {
            let prev = Some(tail.clone());
            let new_tail = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev,
            })));
            tail.borrow_mut().next = new_tail.clone();
            self.tail = new_tail;
        } else {
            let new_node = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev: None,
            })));
            self.head = new_node.clone();
            self.tail = new_node;
        }
    }

    fn tail(&self) -> Option<NodeRef<T>> {
        self.tail.clone()
    }

    /// Warning: this will not check that the provided node belongs to the current list.
    pub fn unlink_node(&mut self, node_to_remove: Option<NodeRef<T>>) {
        let node_to_remove = node_to_remove.unwrap();

        match node_to_remove.borrow().prev.clone() {
            Some(prev) => prev.borrow_mut().next = node_to_remove.borrow().next.clone(),
            None => self.head = node_to_remove.borrow().next.clone(),
        };

        match node_to_remove.borrow().next.clone() {
            Some(next) => next.borrow_mut().prev = node_to_remove.borrow().prev.clone(),
            // if we remove the tail, assign new tail:
            None => self.tail = node_to_remove.borrow().prev.clone(),
        };
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
            last: self.tail.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Iter<T> {
    next: Option<NodeRef<T>>,
    last: Option<NodeRef<T>>,
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.clone() {
            if Rc::ptr_eq(&self.next.clone().unwrap(), &self.last.clone().unwrap()) {
                self.last = None;
                self.next = None;
            } else {
                self.next = next.borrow().next.clone();
            }

            return Some(next);
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(last) = self.last.clone() {
            // Return the tail, and set the second to last element
            // (new tail) as the tail

            // Check if self.next and self.last are the same node. If
            // so, return that node and set them both to None
            if Rc::ptr_eq(&self.next.clone().unwrap(), &self.last.clone().unwrap()) {
                self.last = None;
                self.next = None;
            } else {
                self.last = last.borrow().prev.clone();
            }
            return Some(last);
        } else {
            // iterator is empty:
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
        // println!(
        //     "comparing two nodes: self.head: {:?}, other.head: {:?}",
        //     self, other
        // );
        self.data == other.data && self.next == other.next
    }

    fn ne(&self, other: &Self) -> bool {
        // println!(
        //     "comparing two nodes: self.head: {:?}, other.head: {:?}",
        //     self, other
        // );
        self.data != other.data && self.next == other.next
    }
}

impl<T: PartialEq + std::fmt::Debug> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        // println!(
        //     "comparing two lists: self.head: {:?}, other.head: {:?}",
        //     self.head, other.head
        // );
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
    fn iter_next_back() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let mut iter = list.iter();
        assert_eq!(3, iter.next_back().unwrap().borrow().data);
        assert_eq!(2, iter.next_back().unwrap().borrow().data);
        assert_eq!(1, iter.next_back().unwrap().borrow().data);
        assert_eq!(None, iter.next_back());
    }

    #[test]
    fn iter_double_ended() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);

        let mut iter = list.iter();
        assert_eq!(1, iter.next().unwrap().borrow().data);
        assert_eq!(4, iter.next_back().unwrap().borrow().data);
        assert_eq!(2, iter.next().unwrap().borrow().data);
        assert_eq!(3, iter.next_back().unwrap().borrow().data);
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
        assert_eq!(None, iter.next());
    }
}
