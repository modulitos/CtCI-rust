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

    pub fn push(&mut self, value: T) -> Result<(), &str> {
        if self.is_full() {
            self.grow(self.cap + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("stack is empty! pop aborted");
        }

        self.length -= 1;
        if let Some(value) = self.buf[self.length].clone() {
            Ok(value)
        } else {
            Err("corrupted state of stack!!!")
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
        assert!(s.push(5).is_ok());
    }

    #[test]
    fn pop_empty() {
        let mut s: Stack<u64> = Stack::new();
        assert!(s.pop().is_err());
    }


    #[test]
    fn push_one_pop_one() {
        let mut s: Stack<u64> = Stack::new();
        assert!(s.push(5).is_ok());
        assert_eq!(s.pop(), Ok(5));
    }

    #[test]
    fn push_until_grow() {
        let mut s: Stack<u64> = Stack::new().with_capacity(2).create();
        assert!(s.push(1).is_ok());
        assert!(s.push(2).is_ok());
        assert!(s.push(3).is_ok()); // grow to 4!
        assert!(s.push(4).is_ok());
        assert!(s.push(4).is_ok()); // grow to 8!
        assert!(s.push(4).is_ok());
        assert!(s.push(4).is_ok());
        assert!(s.push(4).is_ok());
        assert!(s.push(4).is_ok()); // grow to 9!
    }

    #[test]
    fn push_then_pop() {
        let mut s: Stack<u64> = Stack::new().with_capacity(2).create();
        assert!(s.push(1).is_ok());
        assert!(s.push(2).is_ok());
        assert!(s.push(3).is_ok()); // grow to 4!
        assert!(s.push(4).is_ok());
        assert!(s.push(5).is_ok()); // grow to 8!
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        assert_eq!(s.pop(), Ok(1));
    }
}
