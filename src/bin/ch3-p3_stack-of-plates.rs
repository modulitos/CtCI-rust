// Imagine a (literal) stack of plates. If the stack gets too high, it
// might topple. Therefore, in real life, we would likely start a new
// stack when the previous stack exceeds some threshold.

// Implement a data structure SetOfStacks that mimics this.
// SetOfStacks should be composed of several stacks and should create
// a new stack once the previous one exceeds capacity. SetOfStacks.
// push () and SetOfStacks. pop () should behave identically to a
// single stack (that is, pop ( ) should return the same values as it
// would if there were just a single stack).

//FOLLOW UP: Implement a function popAt (int index) which performs a
// pop operation on a specific sub-stack.

use cracking::Stack;

struct SetOfStacks<T> {
    stacks: Box<[Stack<T>]>,
}

const NUMBER_OF_STACKS: u8 = 3;
const STACK_MAX_LENGTH: u8 = 3;

impl<T> SetOfStacks<T>
where
    T: std::clone::Clone + std::cmp::PartialOrd,
{
    fn new() -> Self {
        Self {
            stacks: vec![
                Stack::new()
                    .with_capacity(usize::from(STACK_MAX_LENGTH))
                    .is_growable(false)
                    .create();
                usize::from(NUMBER_OF_STACKS)
            ]
            .into_boxed_slice(),
        }
    }

    fn push(&mut self, value: T) {
        if let Some(first_non_full) = self.stacks.iter_mut().find(|stack| !stack.is_full()) {
            first_non_full.push(value);
        } else {
            panic!("all full!");
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(first_non_empty) = self.stacks.iter_mut().rev().find(|stack| !stack.is_empty())
        {
            first_non_empty.pop()
        } else {
            panic!("all empty!!!");
        }
    }

    fn _consolidate_stacks(&mut self) {
        let stacks = self.stacks.iter_mut();
        // The two stacks:
        // 1 2
        // 4 5 6
        // should become:
        // 1 2 3
        // 5 6
        let mut prev_stack_wrapper: Option<&mut Stack<T>> = None;
        for stack in stacks {
            if let Some(prev_stack) = prev_stack_wrapper {
                if !prev_stack.is_full() && !stack.is_empty() {
                    // fill up stack using items from next_stack, starting
                    // at the bottom:
                    let mut temp = Stack::new();
                    while let Some(value) = stack.pop() {
                        temp.push(value);
                    }
                    while let Some(value) = temp.pop() {
                        if !prev_stack.is_full() {
                            prev_stack.push(value);
                        } else {
                            // refill the next stack back up with values:
                            stack.push(value);
                        }
                    }
                }
            }
            prev_stack_wrapper = Some(stack);
        }
    }

    fn pop_at(&mut self, index: u8) -> Option<T> {
        if usize::from(index) >= self.stacks.len() {
            panic!("pop_at: index out of range: {}", index);
        }
        let some_item = self.stacks[usize::from(index)].pop();
        self._consolidate_stacks();
        some_item
    }
}

mod test {
    use super::*;

    #[test]
    fn push_then_pop() {
        let mut s: SetOfStacks<u64> = SetOfStacks::new();
        s.push(1);
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn push_first_stack_beyond_capacity() {
        let mut s: SetOfStacks<u64> = SetOfStacks::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn push_second_stack_beyond_capacity() {
        let mut s: SetOfStacks<u64> = SetOfStacks::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);
        s.push(6);
        s.push(7);
        assert_eq!(s.pop(), Some(7));
        assert_eq!(s.pop(), Some(6));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn pop_at() {
        let mut s: SetOfStacks<u64> = SetOfStacks::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        assert_eq!(s.pop_at(0), Some(3));
    }

    #[test]
    fn push_then_pop_after_pop_at() {
        let mut s: SetOfStacks<u64> = SetOfStacks::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        assert_eq!(s.pop_at(0), Some(3));  // this should shift 4 down to the first stack
        s.push(5);
        s.push(6);
        s.push(7);
        s.push(8);
        assert_eq!(s.pop(), Some(8));
        assert_eq!(s.pop(), Some(7));
        assert_eq!(s.pop(), Some(6));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
    }

    #[test]
    fn push_then_pop_after_multiple_pop_at() {
        let mut s: SetOfStacks<u64> = SetOfStacks::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        assert_eq!(s.pop_at(0), Some(3));  // this should shift 4 down to the first stack
        s.push(5);
        s.push(6);
        s.push(7);
        s.push(8);
        assert_eq!(s.pop_at(1), Some(7));  // this should shift 8 down to the second stack
        assert_eq!(s.pop_at(0), Some(4));  // this should shift 5 down to the first stack
        assert_eq!(s.pop(), Some(8));
        assert_eq!(s.pop(), Some(6));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
    }
}
