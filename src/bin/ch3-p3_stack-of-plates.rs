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

const SET_LENGTH:u8 = 3;

impl<T> SetOfStacks<T>
where
    T: std::clone::Clone + std::cmp::PartialOrd,
{
    fn new() -> Self {
        Self {
            stacks: vec![Stack::new(); usize::from(SET_LENGTH)].into_boxed_slice(),
        }
    }

    fn push(&mut self, value: T) {
        self.stacks[0].push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.stacks[0].pop()
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

}
