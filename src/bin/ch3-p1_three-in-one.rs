// Describe how you could use a single array to implement three stacks

use std::boxed::Box;
use std::cmp;

type Node<T> = Option<T>;

const DEFAULT_CAP: usize = 10;

pub struct ThreeStacks<T> {
    buf: Box<[Node<T>]>, // this is our single array.
    cap: usize,
    indexes: [usize; 3],
}

impl<T> ThreeStacks<T>
where
    T: std::clone::Clone,
{
    pub fn new() -> Self {
        Self {
            buf: vec![None; DEFAULT_CAP].into_boxed_slice(),
            cap: DEFAULT_CAP,
            indexes: [0, DEFAULT_CAP / 3, (DEFAULT_CAP / 3) * 2],
        }
    }

    pub fn push(&mut self, value: T, stack: usize) -> Result<(), &str> {
        // TODO: If stack is full, we'll need to grow and copy/paste the data over.
        if stack > self.indexes.len() {
            return Err("wrong stack reference!");
        }
        if self.is_full(stack) {
            return Err("out of room! Push aborted.");
        }
        self.buf[self.indexes[stack]] = Some(value);
        self.indexes[stack] += 1;
        Ok(())
    }

    pub fn pop(&mut self, stack: usize) -> Result<T, &str> {
        // if stack is empty, return error type:
        if self.is_empty(stack) {
            return Err("stack is empty! pop aborted");
        }

        self.indexes[stack] -= 1;
        if let Some(value) = self.buf[self.indexes[stack]].clone() {
            Ok(value)
        } else {
            Err("something is wrong")
        }
    }

    fn is_full(&self, stack: usize) -> bool {
        let index = self.indexes[stack];
        println!("index: {}", index);
        println!("index max: {},", ((stack + 1) * ((self.cap + 1) / 3)));
        index >= (stack + 1) * ((self.cap + 1) / 3)
    }

    fn is_empty(&self, stack: usize) -> bool {
        let index = self.indexes[stack];
        println!("index: {}", index);
        println!("index min: {},", (stack * (self.cap / 3)));
        index <= stack * (self.cap / 3)
    }

    // TODO: Use this approach to grow the array.
    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);
        println!("newcap: {}", new_cap);
        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;
        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }
}

mod test {
    use super::*;

    #[test]
    fn create() {
        assert!(true);
        let a: ThreeStacks<u64> = ThreeStacks::new();
        assert!(true);
    }

    #[test]
    fn push_one() {
        let mut s: ThreeStacks<u64> = ThreeStacks::new();
        assert!(s.push(5, 0).is_ok());
    }

    #[test]
    fn push_too_many_gives_err() {
        let mut s: ThreeStacks<u64> = ThreeStacks::new();
        assert!(s.push(1, 0).is_ok());
        assert!(s.push(2, 0).is_ok());
        assert!(s.push(3, 0).is_ok());
        assert!(s.push(4, 0).is_err());

        assert!(s.push(1, 1).is_ok());
        assert!(s.push(2, 1).is_ok());
        assert!(s.push(3, 1).is_ok());
        assert!(s.push(4, 1).is_err());

        assert!(s.push(1, 2).is_ok());
        assert!(s.push(2, 2).is_ok());
        assert!(s.push(3, 2).is_ok());
        assert!(s.push(4, 2).is_err());
    }

    #[test]
    fn pop_empty_gives_err() {
        let mut s: ThreeStacks<u64> = ThreeStacks::new();
        assert!(s.pop(0).is_err());
        assert!(s.pop(1).is_err());
        assert!(s.pop(2).is_err());
    }
}
