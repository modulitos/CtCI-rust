// Queue via Stacks: Implement a MyQueue class which implements a
// queue using two stacks.

use cracking::Stack;

struct MyQueue<T> {
    incoming: Stack<T>,
    outgoing: Stack<T>,
}

impl<T> MyQueue<T>
where
    T: std::clone::Clone + std::cmp::PartialOrd,
{
    fn new() -> Self {
        Self {
            incoming: Stack::new(),
            outgoing: Stack::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.incoming.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        if self.outgoing.is_empty() && !self.incoming.is_empty() {
            while let Some(value) = self.incoming.pop() {
                self.outgoing.push(value);
            }
        }
        self.outgoing.pop()
    }
}

mod test {
    use super::*;

    #[test]
    fn pop_empty() {
        let mut s: MyQueue<u64> = MyQueue::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn push_then_pop() {
        let mut s: MyQueue<u64> = MyQueue::new();
        s.push(1);
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn push_then_pop_push_pop() {
        let mut s: MyQueue<u64> = MyQueue::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(1));
        s.push(4);
        s.push(5);
        s.push(6);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(6));
    }
}
