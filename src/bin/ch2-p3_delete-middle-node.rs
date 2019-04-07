// Delete Middle Node: Implement an algorithm to delete a node in the
// middle (i.e., any node but the first and last node, not necessarily
// the exact middle) of a singly linked list, given only access to
// that node.

// EXAMPLE Input: the node c from the linked list a->b->c->d->e->f
// Result: nothing is returned, but the new linked list looks like
// a->b->d->e->f

use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ data: {:?}, next: {:?} }}", self.data, self.next)
    }
}

impl<T> Node<T> {
    fn tail(node: &NodeRef<T>) -> Option<NodeRef<T>> {
        if let Some(cur) = node.borrow().next.as_ref().cloned() {
            return Node::tail(&cur.clone());
        }
        Some(node.clone())
    }
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: std::clone::Clone,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&mut self, new_value: T) {
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

    fn tail(&self) -> Option<NodeRef<T>> {
        if let Some(cur) = self.head.as_ref().cloned() {
            if cur.borrow().next.is_none() {
                return Some(cur.clone());
            } else {
                return Node::tail(&cur.clone());
            }
        }
        None
    }

    /// Warning: this will not check that the provided node belongs to the current list.
    fn _unlink_node(&mut self, node_to_remove: Option<NodeRef<T>>) {
        let node_to_remove = node_to_remove.unwrap();

        for n in self.iter() {
            let mut borrowed_node = n.borrow_mut();
            if let Some(next) = borrowed_node.next.clone() {
                if Rc::ptr_eq(&next, &node_to_remove) {
                    borrowed_node.next = node_to_remove.borrow_mut().next.take();
                    break;
                }
            } else if Rc::ptr_eq(&n, &node_to_remove) {
                // handle case when node_to_remove is the only element
                // in the list
                self.head = None;
            }
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }

    fn delete_middle_node(&mut self) {
        let mut middle_node: Option<NodeRef<T>> = None;
        for (n, _) in self.iter().enumerate() {
            // using "tortoise/hare" method:
            // middle_node is the tortoise
            // _ is the hare
            if n % 2 == 0 {
                if let Some(node) = middle_node {
                    middle_node = node.borrow().next.clone();
                } else {
                    middle_node = self.head.clone();
                }
            }
        }
        // delete the middle node:
        if middle_node.is_some() {
            self._unlink_node(middle_node);
        }
    }
}

struct Iter<T> {
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
        println!(
            "comparing two nodes: self.head: {:?}, other.head: {:?}",
            self, other
        );
        self.data == other.data && self.next == other.next
    }

    fn ne(&self, other: &Self) -> bool {
        println!(
            "comparing two nodes: self.head: {:?}, other.head: {:?}",
            self, other
        );
        self.data != other.data && self.next == other.next
    }
}

impl<T: PartialEq + std::fmt::Debug> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        println!(
            "comparing two lists: self.head: {:?}, other.head: {:?}",
            self.head, other.head
        );
        self.head == other.head
    }

    fn ne(&self, other: &Self) -> bool {
        println!(
            "comparing two lists: self.head: {:?}, other.head: {:?}",
            self.head, other.head
        );
        self.head != other.head
    }
}

impl<T: Eq + std::fmt::Debug> Eq for LinkedList<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_middle_node() {
        let mut list1 = LinkedList::<usize>::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);
        list1.append(4);

        let mut list2 = LinkedList::<usize>::new();
        list2.append(1);
        list2.append(3);
        list2.append(4);

        list1.delete_middle_node();

        assert_eq!(list1, list2);

        let mut list1 = LinkedList::<usize>::new();
        list1.append(1);

        let list2 = LinkedList::<usize>::new();

        list1.delete_middle_node();

        assert_eq!(list1, list2);

        let mut list1 = LinkedList::<usize>::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);

        let mut list2 = LinkedList::<usize>::new();
        list2.append(1);
        list2.append(3);

        list1.delete_middle_node();

        assert_eq!(list1, list2);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
