use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hasher;
use std::rc::Rc;
use std::fmt;
// Inspired by:
// https://raw.githubusercontent.com/brndnmtthws/cracking-the-coding-interview-rust/master/src/bin/c02p01.rs
// but with some improvements:

//  * implements Eq and PartialEq traits, instead of a
//  "has_duplicates" function.
//  * uses an _unlink_node method on LinkedList, instead of on the
//  Node struct, which is more consistent with the official library


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

struct Iter<T> {
    next: Option<NodeRef<T>>,
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

    /// Warning: this will not check that the provided node belongs to the current list.
    fn _unlink_node(&mut self, node_to_remove: Option<NodeRef<T>>) {
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

    fn remove_duplicates(&mut self) {
        let mut set: HashSet<u64> = HashSet::new();
        for node in self.iter() {
            let mut s = DefaultHasher::new();
            {
                let data = &node.borrow().data;
                data.hash(&mut s);
            }
            let hash = s.finish();
            if set.contains(&hash) {
                self._unlink_node(Some(node));
            }
            set.insert(hash);
        }
    }
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
        println!("comparing two nodes: self.head: {:?}, other.head: {:?}", self, other);
        self.data == other.data && self.next == other.next
    }

    fn ne(&self, other: &Self) -> bool {
        println!("comparing two nodes: self.head: {:?}, other.head: {:?}", self, other);
        self.data != other.data && self.next == other.next
    }
}

impl<T: PartialEq + std::fmt::Debug > PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        println!("comparing two lists: self.head: {:?}, other.head: {:?}", self.head, other.head);
        self.head == other.head
    }

    fn ne(&self, other: &Self) -> bool {
        println!("comparing two lists: self.head: {:?}, other.head: {:?}", self.head, other.head);
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
    fn test_list_remove_duplicates() {
        let mut list1 = LinkedList::<String>::new();
        list1.append(String::from("item1"));
        list1.append(String::from("item2"));

        list1.remove_duplicates();
        let mut list2 = LinkedList::<String>::new();
        list2.append(String::from("item1"));
        list2.append(String::from("item2"));
        assert_eq!(list1, list2);

        list1.append(String::from("item1"));
        list1.remove_duplicates();

        assert_eq!(list1, list2);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    list.remove_duplicates();
}
