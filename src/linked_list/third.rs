use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

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
        List { head: None }
    }

    pub fn append(&mut self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
                // next: self.head.take(), // equivalent
                // next: self.head,  // cannot move out of borrowed
            })),
        }
    }
    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    // pub fn remove_dups(&mut self) -> &Self {
    //     let mut set: HashSet<T> = HashSet::new();
    //     let mut curr: Link<T> = self.head.take();
    //     let mut prev: Link<T> = None;
    //     while curr.is_some() {
    //         if set.contains(&curr.as_ref().unwrap().elem) {
    //             // Can't transfer ownership out of an Rc???
    //             // "cannot assign to data in a `&` reference"???
    //             prev.unwrap().next = curr.as_ref().and_then(|node| node.next.clone());
    //             curr = curr.as_ref().and_then(|node| node.next.clone());
    //         } else {
    //             set.insert(curr.as_ref().unwrap().elem);
    //             curr = curr.as_ref().and_then(|node| node.next.clone());
    //             prev = curr.take();
    //         }
    //     }

    //     &*self // make immutable before returning
    // }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
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

// Iter implementation is identical to second

// Note that we can't implement IntoIter or IterMut for this type. We
// only have shared access to elements.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn eq() {
        let mut list = List::new().append(1).append(2).append(3);
        let mut list2 = List::new().append(1).append(2).append(3);

        assert_eq!(list, list2);
        list2 = list2.tail();
        assert_ne!(list, list2);
        list = list.tail();
        assert_eq!(list, list2);
    }
}
