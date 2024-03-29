// Sort Stack: Write a program to sort a stack such that the smallest
// items are on the top. You can use an additional temporary stack,
// but you may not copy the elements into any other data structure
// (such as an array). The stack supports the following operations:
// push, pop, peek, and isEmpty.

extern crate cracking;
use cracking::Stack;

pub trait SortStack<T> {
    fn sort(&mut self);
}

impl<T> SortStack<T> for Stack<T>
where
    T: std::clone::Clone + std::marker::Copy + std::cmp::PartialOrd,
{
    fn sort(&mut self) {
        // we want this stack to be ordered with the smallest items on
        // the bottom:
        let mut temp = Stack::<T>::new();
        // pop items off of this stack until we find an item that is
        // out of order (less than the previous)
        while let Some(curr) = self.pop() {
            if let Some(prev) = temp.peek() {
                if curr >= prev {
                    // curr is in the correct order - move along!
                    temp.push(curr);
                } else {
                    // curr is out of order!

                    // Pop the new stack onto the old one until next
                    // is at the bottom or larger than the next item:
                    // loop {
                    while let Some(prev) = temp.peek() {
                        if curr < prev {
                            self.push(temp.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    // then put the 'curr' item into the new stack:
                    temp.push(curr);
                    // (no need to re-update curr)
                }
            } else {
                temp.push(curr);
            }
        }
        // pop all of temp back into our stack:
        while let Some(value) = temp.pop() {
            self.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_2() {
        let mut s = Stack::<u64>::new();
        s.push(1);
        s.push(2);
        s.sort();
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(2));
    }

    #[test]
    fn sort_3() {
        let mut s = Stack::<u64>::new();
        s.push(1);
        s.push(3);
        s.push(2);
        s.sort();
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(3));
    }

    #[test]
    fn sort_4() {
        let mut s = Stack::<u64>::new();
        s.push(1);
        s.push(3);
        s.push(2);
        s.push(4);
        s.sort();
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(4));
    }

    #[test]
    fn sort_5() {
        let mut s = Stack::<u64>::new();
        s.push(1);
        s.push(3);
        s.push(5);
        s.push(2);
        s.push(4);
        s.sort();
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(5));
    }
}
