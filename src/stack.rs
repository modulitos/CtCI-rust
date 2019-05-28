use std::boxed::Box;
use std::cmp;

type Node<T> = Option<T>;

pub struct Stack<T> {
    buf: Box<[Node<T>]>,
    cap: usize,
    length: usize,
}

const DEFAULT_CAP: usize = 10;

impl<T> Stack<T>
where
    T: std::clone::Clone,
{
    pub fn new() -> Self {
        Self {
            buf: vec![None; DEFAULT_CAP].into_boxed_slice(),
            cap: DEFAULT_CAP,
            length: 0,
        }
    }

    // Using the builder pattern to allow for optional arguments.
    pub fn with_capacity(mut self, custom_cap: usize) -> Self {
        self.cap = custom_cap;
        self.buf = vec![None; custom_cap].into_boxed_slice();
        self
    }

    pub fn create(self) -> Self {
        self
    }

    pub fn push(&mut self, value: T) {
        if self.is_full() {
            self.grow(self.cap + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        if let Some(value) = self.buf[self.length - 1].clone() {
            Some(value)
        } else {
            panic!("Stack.peek: corrupted state of stack!!!")
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.length -= 1;
        if let Some(value) = self.buf[self.length].clone() {
            Some(value)
        } else {
            panic!("Stack.pop: corrupted state of stack!!!")
        }
    }

    fn is_full(&self) -> bool {
        self.length == self.cap
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        println!("old_cap: {}", old_cap);
        let mut new_cap = old_cap << 1; // double the size of the cap
        println!("newcap: {}", new_cap);
        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;
        println!("self.cap: {}", self.cap);
        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }
}

mod test {
    use super::*;

    #[test]
    fn create() {
        let _: Stack<u64> = Stack::new();
        assert!(true);
    }

    #[test]
    fn create_with_capacity() {
        let _: Stack<u64> = Stack::new().with_capacity(3).create();
        assert!(true);
    }

    #[test]
    fn push_one() {
        let mut s: Stack<u64> = Stack::new();
        s.push(5);
    }

    #[test]
    fn pop_empty() {
        let mut s: Stack<u64> = Stack::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn peek_empty() {
        let mut s: Stack<u64> = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn push_then_peek() {
        let mut s: Stack<u64> = Stack::new();
        s.push(1);
        assert_eq!(s.peek(), Some(1));
        assert_eq!(s.peek(), Some(1));
    }

    #[test]
    fn push_one_pop_one() {
        let mut s: Stack<u64> = Stack::new();
        s.push(5);
        assert_eq!(s.pop(), Some(5));
    }

    #[test]
    fn push_until_grow() {
        let mut s: Stack<u64> = Stack::new().with_capacity(2).create();
        s.push(1);
        s.push(2);
        s.push(3); // grow to 4!
        s.push(4);
        s.push(4); // grow to 8!
        s.push(4);
        s.push(4);
        s.push(4);
        s.push(4); // grow to 9!
    }

    #[test]
    fn push_then_pop() {
        let mut s: Stack<u64> = Stack::new().with_capacity(2).create();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
    }
}
