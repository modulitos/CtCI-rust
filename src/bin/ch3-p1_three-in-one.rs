use std::boxed::Box;
use std::cmp;

type Node<T> = Option<T>;

const DEFAULT_CAP: usize = 10;

pub struct ThreeStacks<T> {
    buf: Box<[Node<T>]>,
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

    pub fn push(&mut self, value: T, stack: usize) -> Result<(), &str>{
        // TODO: If stack is full, we'll need to grow and copy/paste the data over.
        if stack > self.indexes.len() {
            return Err("wrong stack reference!");
        }
        let index = self.indexes[stack];
        if index > (index * ((self.cap + 1) / 3)) {
            return Err("out of room!");
        }
        self.buf[index] = Some(value);
        return Ok(());
    }

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
    fn insert_one_item() {
        let mut s: ThreeStacks<u64> = ThreeStacks::new();
        s.push(5, 0);
        assert!(true);
    }
}
