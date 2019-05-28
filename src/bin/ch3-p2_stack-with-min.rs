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
            if value < curr_min {
                self.mins.push(value);
            }
        } else {
            self.mins.push(value);
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
}
