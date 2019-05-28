// Stack Min: How would you design a stack which, in addition to push and pop, has a function min which returns the minimum element? Push, pop and min should all operate in 0(1) time.
use cracking::Stack;

struct MinStack<T> {
    stack: Stack<T>,
    mins: Stack<T>,
}

impl<T> MinStack<T>
where
    T: std::clone::Clone + std::cmp::PartialOrd,
{
    fn new() -> Self {
        Self {
            stack: Stack::new(),
            mins: Stack::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.stack.push(value.clone());
        if let Some(curr_min) = self.mins.peek() {
            if value <= curr_min {
                self.mins.push(value);
            }
        } else {
            self.mins.push(value);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(value) = self.stack.pop() {
            // Invariant: value can never be less than the last item
            // in self.mins:
            if value == self.mins.peek().unwrap() {
                // Invariant: self.mins can never be empty here:
                self.mins.pop().unwrap();
            }
            Some(value)
        } else {
            None
        }
    }

    fn get_min(&self) -> Option<T> {
        self.mins.peek()
    }
}

mod test {
    use super::*;

    #[test]
    fn get_min() {
        let mut s: MinStack<u64> = MinStack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.get_min(), Some(1));
    }

    #[test]
    fn get_min_after_pop() {
        let mut s: MinStack<u64> = MinStack::new();
        s.push(3);
        s.push(2);
        s.push(1);
        assert_eq!(s.get_min(), Some(1));
        s.pop();
        assert_eq!(s.get_min(), Some(2));
        s.pop();
        assert_eq!(s.get_min(), Some(3));
        s.push(1);
        assert_eq!(s.get_min(), Some(1));
        s.push(2);
        assert_eq!(s.get_min(), Some(1));
    }

    #[test]
    fn push_then_get_identical_mins() {
        let mut s: MinStack<u64> = MinStack::new();
        s.push(3);
        s.push(1);
        s.push(2);
        s.push(1);
        assert_eq!(s.get_min(), Some(1));
        s.pop();
        assert_eq!(s.get_min(), Some(1));
        s.pop();
        assert_eq!(s.get_min(), Some(1));
        s.pop();
        assert_eq!(s.get_min(), Some(3));
        s.push(2);
        s.push(2);
        assert_eq!(s.get_min(), Some(2));
        s.pop();
        assert_eq!(s.get_min(), Some(2));
        s.pop();
        assert_eq!(s.get_min(), Some(3));
    }
}
