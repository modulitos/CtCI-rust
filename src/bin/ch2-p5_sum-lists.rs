// Sum Lists: You have two numbers represented by a linked list, where
// each node contains a single digit. The digits are stored in reverse
// order, such that the 1 's digit is at the head of the list. Write a
// function that adds the two numbers and returns the sum as a linked
// list.

// EXAMPLE:
// Input: (7-) 1 -) 6) + (5 -) 9 -) 2).That is, 617 + 295.
// Output: (2 -) 1 -) 9). That is, 912.

// FOLLOW UP Suppose the digits are
// stored in forward order. Repeat the above problem.
// EXAMPLE Input:
// (6 -) 1 -) 7) + (2 -) 9 -) 5).That is, 617 + 295.
// Output: 9 -) 1 -) 2).
// That is, 912.

extern crate cracking;

use cracking::LinkedList;

pub trait SumLists<T> {
    fn sum_lists(&self, other: LinkedList<T>) -> LinkedList<T>;
}

impl<T> SumLists<T> for LinkedList<T> {
    fn sum_lists(&self, other: LinkedList<T>) -> LinkedList<T> {
        other
    }
}

mod test {
    use super::*;

    #[test]
    fn linked_list_api() {
        let mut list = LinkedList::new();
        list.append(5);
        list.append(9);
        list.append(2);
        let mut list2 = LinkedList::new();
        list2.append(5);
        list2.append(9);
        list2.append(2);

        assert_eq!(list, list2);
    }
    #[test]
    fn add_lists() {
        // 617 + 295 = 912
        let mut list = LinkedList::new();
        list.append(7);
        list.append(1);
        list.append(6);
        let mut list2 = LinkedList::new();
        list2.append(5);
        list2.append(9);
        list2.append(2);

        let mut sum_list = LinkedList::new();
        sum_list.append(2);
        sum_list.append(1);
        sum_list.append(9);

        assert_eq!(list.sum_lists(list2), sum_list);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
