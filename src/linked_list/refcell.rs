use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NodeRef<T>>,
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
        Self { head: None }
    }

    pub fn prepend(&mut self, new_value: T) {
        self.head = Some(Rc::new(RefCell::new(Node {
            data: new_value,
            next: self.head.take(),
            prev: None,
        })));
    }

    pub fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail() {
            let prev = Some(tail.clone());
            let new_tail = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev,
            })));
            tail.borrow_mut().next = new_tail;
        } else {
            let new_node = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev: None,
            })));
            self.head = new_node;
        }
    }

    fn tail(&self) -> Option<NodeRef<T>> {
        for node in self.iter() {
            if let None = node.borrow().next {
                return Some(node.clone());
            }
        }
        None
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
            _ => (),
        };
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }
}

pub struct Iter<T> {
    next: Option<NodeRef<T>>,
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
            return Some(cur.clone());
        }
        None
    }
    // TODO: consider this for reverse iteration - maybe using doubly LL?
    // fn next_back(&mut self) -> Option<Self::Item> {
    //     // TODO
    // }
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
        list2 = LinkedList { head: list2.tail() };
        assert_ne!(list, list2);
        list = LinkedList { head: list.tail() };
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


}
