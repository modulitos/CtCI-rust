use std::boxed::Box;
use std::cmp;

type Node<T> = Option<T>;

pub struct ArrayList<T> {
    buf: Box<[Node<T>]>,
    cap: usize,
    pub length: usize,
}

impl<T> ArrayList<T>
where
    T: std::clone::Clone,
{
    pub fn new() -> Self {
        Self {
            buf: vec![None; 10].into_boxed_slice(),
            cap: 10,
            length: 0,
        }
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);
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
    fn test_array_resizing() {
        assert!(true);
        let a: ArrayList<u64> = ArrayList::new();
        assert!(true);
    }
}
