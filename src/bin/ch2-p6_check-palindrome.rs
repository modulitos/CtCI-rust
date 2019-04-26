// Palindrome: Implement a function to check if a linked list is a
// palindrome.
extern crate cracking;
use cracking::LinkedList;
use std::vec::Vec;

pub trait CheckPalindrome<T> {
    fn is_palindrome(&self) -> bool;
    fn is_palindrome_iterative(&self) -> bool;
}

// Assumes that LinkedList has implemented a doubleendediterator
// (next_back method)
impl<T> CheckPalindrome<T> for LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    fn is_palindrome(&self) -> bool {
        let mut iter = self.iter();
        while let Some(next) = iter.next() {
            // println!("checking next: {:?}", next);
            if let Some(last) = iter.next_back() {
                // println!("comparing next with last: {:?}", last);
                if next.borrow().data != last.borrow().data {
                    return false;
                }
            }
        }
        true
    }

    fn is_palindrome_iterative(&self) -> bool {
        let mut iter_fast = self.iter();
        let mut iter_slow = self.iter();
        let mut stack: Vec<T> = Vec::new();
        let mut is_odd = false;
        while let Some(_) = iter_fast.next() {
            let next_slow = iter_slow.next().unwrap();
            stack.push(next_slow.borrow().data.clone());
            if let None = iter_fast.next() {
                // we have an odd number of nodes
                is_odd = true;
            }
        }
        // now we know slow iterator is at node len/2
        if is_odd {
            stack.pop();
        }
        while let Some(next) = iter_slow.next() {
            let next_data = stack.pop().unwrap();
            if next_data != next.borrow().data {
                return false;
            }
        }
        true
    }
}

mod test {
    use super::*;

    #[test]
    fn check_palindrome() {
        let mut list = LinkedList::new();
        list.append('a');
        list.append('b');
        list.append('c');
        assert!(!list.is_palindrome());
        list.append('c');
        list.append('b');
        list.append('a');
        assert!(list.is_palindrome());
    }

    #[test]
    fn check_palindrome_iterative() {
        let mut list = LinkedList::new();
        list.append('a');
        list.append('b');
        list.append('c');
        assert!(!list.is_palindrome_iterative());
        list.append('c');
        list.append('b');
        list.append('a');
        assert!(list.is_palindrome_iterative());
    }
    #[test]
    fn check_odd_palindrome() {
        let mut list = LinkedList::new();
        list.append('a');
        list.append('b');
        list.append('c');
        list.append('b');
        list.append('a');
        assert!(list.is_palindrome());
        list.append('a');
        assert!(!list.is_palindrome());
    }
    #[test]
    fn check_odd_palindrome_iterative() {
        let mut list = LinkedList::new();
        list.append('a');
        list.append('b');
        list.append('c');
        list.append('b');
        list.append('a');
        assert!(list.is_palindrome_iterative());
        list.append('a');
        assert!(!list.is_palindrome_iterative());
    }

}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
