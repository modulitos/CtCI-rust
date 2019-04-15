// Partition: Write code to partition a linked list around a value x,
// such that all nodes less than x come before all nodes greater than
// or equal to x. If x is contained within the list, the values of x
// only need to be after the elements less than x (see below). The
// partition element x can appear anywhere in the "right partition";
// it does not need to appear between the left and right partitions.

// EXAMPLE Input: 3->5->8->5->10->2->1 [partition = 5)
// Output: 3->1->2->10->5->5->8

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

impl<T> LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
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
        for node in self.iter() {
            if let None = node.borrow().next {
                return Some(node.clone());
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

    fn partition(&mut self, partition_value: T) {
        let mut values_to_append: Vec<T> = Vec::new();
        for node in self.iter() {
            // if the current value is equal or higher than our
            // partition, remove the node and append its value to the
            // end
            let current_value = node.borrow().data.clone();
            println!("iterating over current value: {:?}", current_value);
            if current_value >= partition_value {
                self._unlink_node(Some(node));
                values_to_append.push(current_value);
            }
        }
        for value in values_to_append.into_iter() {
            self.append(value);
        }
    }

    fn verify_partition(&mut self, value: T) -> bool {
        let mut is_above = false;
        for node in self.iter() {
            let curr_value = node.borrow().data.clone();
            if curr_value < value && is_above {
                return false;
            }
            if curr_value >= value {
                is_above = true;
            }
        }
        true
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
    fn test_partition() {
        // 3->5->8->5->10->2->1
        let mut list1 = LinkedList::<usize>::new();
        list1.append(3);
        list1.append(5);
        list1.append(8);
        list1.append(5);
        list1.append(10);
        list1.append(2);
        list1.append(1);

        assert_eq!(list1.verify_partition(5), false);

        list1.partition(5);

        assert_eq!(list1.verify_partition(5), true);

        // 3->1->2->10->5->5->8
        let mut list2 = LinkedList::<usize>::new();
        list2.append(3);
        list2.append(1);
        list2.append(2);
        list2.append(10);
        list2.append(5);
        list2.append(5);
        list2.append(8);
        assert_eq!(list2.verify_partition(5), true);
        list2.partition(5);

        assert_eq!(list2.verify_partition(5), true);

        let mut list3 = LinkedList::<usize>::new();
        list3.append(3);
        list3.append(1);
        list3.append(2);
        list3.append(10);
        list3.append(5);
        list3.append(5);
        list3.append(8);
        list3.append(11);
        list3.append(2);
        list3.append(4);
        list3.append(6);
        assert_eq!(list3.verify_partition(7), false);
        list3.partition(7);

        assert_eq!(list3.verify_partition(7), true);

    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
