use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::hash::Hasher;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,
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

    fn remove(&mut self) {
        if let Some(ref prev) = self.prev {
            if let Some(ref next) = self.next {
                next.borrow_mut().prev = Some(prev.clone());
                prev.borrow_mut().next = Some(next.clone());
            } else {
                prev.borrow_mut().next = None;
            }
        }
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
            let prev = Some(tail.clone());
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev: None,
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

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }

    fn kth_to_last(&self, k: usize) -> T {
        // use "tortoise/hare" method:
        let mut kth_last_node: Option<NodeRef<T>> = None;
        for (n, _) in self.iter().enumerate() {  // _ is the hare
            if n == k {
                kth_last_node = self.head.clone();
            } else if n > k {
                kth_last_node = kth_last_node.unwrap().borrow().next.clone();
            }
        }
        kth_last_node.unwrap().borrow().data.clone()
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
    fn kth_node() {
        let mut list = LinkedList::<usize>::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);

        assert_eq!(list.kth_to_last(1), 3);
        assert_eq!(list.kth_to_last(2), 2);
        assert_eq!(list.kth_to_last(0), 4);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
