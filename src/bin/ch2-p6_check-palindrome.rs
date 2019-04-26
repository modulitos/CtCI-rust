// Palindrome: Implement a function to check if a linked list is a
// palindrome.
extern crate cracking;
use cracking::LinkedList;


pub trait CheckPalindrome<T> {
    fn is_palindrome(&self) -> bool;
}

// Assumes that both lists are of the same size.
impl<T> CheckPalindrome<T> for LinkedList<T> {
    fn is_palindrome(&self) -> bool {
        false
    }
}

mod test {
    use super::*;

    #[test]
    fn eq() {
        let mut list = LinkedList::new();
        list.append('a');
        list.append('b');
        list.append('c');
        assert!(!list.is_palindrome());
    }

}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
