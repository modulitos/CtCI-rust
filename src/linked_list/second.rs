// use std::mem;

// List is public to avoid having to make Node public

#[derive(Debug)]
pub struct List<T> {
    pub head: Link<T>,
}

// Enums are public by default:
pub type Link<T> = Option<(Box<Node<T>>)>;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// Iterator for returning references to our values:
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            // same as above, but using deref coercion via turbofish:
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &**node);
            &node.elem
        })
    }
}

// Iterator returning mutable references:
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            // same as above, but using deref coercion via turbofish:
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &**node);
            &mut node.elem
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // lifetimes elided here:
    pub fn iter(&self) -> Iter<T> {
        // pub fn iter(&self) -> Iter<'_, T> {
        // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // To avoid moving self.head, we use replace to transfer
            // ownership of the value at self.head, and replace
            // self.head with Link
            // next: mem::replace(&mut self.head, None), // same as:
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        // match mem::replace(&mut self.head, None) {
        // match self.head.take() {
        //     None => None,
        //     Some(boxed_node) => {
        //         // Deref helps us access node.elem after node is moved.
        //         // Might not be needed in future Rust versions.
        //         let node = *boxed_node;
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
        // self.head.as_ref().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            // cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
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
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // type of value is &mut i32 here, but we won't specify that
        // or else the value will get moved automatically
        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
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
    }
}
