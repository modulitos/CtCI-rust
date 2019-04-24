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

// Q: can we assume both numbers have the same number of digits?
// EXAMPLE:
// Input: (9-) 9 -) 9) + (1 -) 0 -) 0).That is, 999 + 001
// Output: (0 -) 0 -) 0 -) 1). That is, 1000.

extern crate cracking;
use cracking::LinkedList;

pub trait SumLists<T> {
    fn sum_lists(&self, other: &LinkedList<T>) -> LinkedList<T>;
    fn sum_lists_reversed(&self, other: &LinkedList<T>) -> LinkedList<T>;
}

// Assumes that both lists are of the same size.
impl SumLists<i32> for LinkedList<i32> {
    fn sum_lists(&self, other: &LinkedList<i32>) -> LinkedList<i32> {
        let mut new_list = LinkedList::new();
        let mut overflow = 0;
        for (node, other_node) in self.iter().zip(other.iter()) {
            let this_data = node.borrow().data;
            let other_data = other_node.borrow().data;
            println!("node: {:?}, other_node: {:?}", this_data, other_data);
            let digits = (this_data + other_data + overflow) % 10;
            overflow = (this_data + other_data + overflow) / 10;
            println!("digits: {}, overflow: {}", digits, overflow);
            new_list.append(digits);
        }
        // Add on an extra node, if there is still an overflow:
        if overflow > 0 {
            new_list.append(overflow);
        }
        new_list
    }

    fn sum_lists_reversed(&self, other: &LinkedList<i32>) -> LinkedList<i32> {
        let mut new_list = LinkedList::new();
        let mut overflow = 0;

        let self_digits = self.iter().map(|node| node.borrow().data).collect::<Vec<i32>>();
        let other_digits = other.iter().map(|node| node.borrow().data).collect::<Vec<i32>>();
        for (this_data, other_data) in self_digits.iter().rev().zip(other_digits.iter().rev()) {
            println!("node: {:?}, other_node: {:?}", this_data, other_data);
            let digits = (this_data + other_data + overflow) % 10;
            overflow = (this_data + other_data + overflow) / 10;
            println!("digits: {}, overflow: {}", digits, overflow);
            new_list.prepend(digits);
        }
        // Add on an extra node, if there is still an overflow:
        if overflow > 0 {
            new_list.prepend(overflow);
        }
        new_list
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

        assert_eq!(list.sum_lists(&list2), sum_list);

        // 999 + 1 = 1000
        let mut list = LinkedList::new();
        list.append(9);
        list.append(9);
        list.append(9);
        let mut list2 = LinkedList::new();
        list2.append(1);
        list2.append(0);
        list2.append(0);

        let mut sum_list = LinkedList::new();
        sum_list.append(0);
        sum_list.append(0);
        sum_list.append(0);
        sum_list.append(1);

        assert_eq!(list.sum_lists(&list2), sum_list);
    }

    #[test]
    fn add_lists_reversed() {
        // 617 + 295 = 912
        let mut list = LinkedList::new();
        list.append(6);
        list.append(1);
        list.append(7);
        let mut list2 = LinkedList::new();
        list2.append(2);
        list2.append(9);
        list2.append(5);

        let mut sum_list = LinkedList::new();
        sum_list.append(9);
        sum_list.append(1);
        sum_list.append(2);

        assert_eq!(list.sum_lists_reversed(&list2), sum_list);

        let mut list = LinkedList::new();
        list.append(9);
        list.append(9);
        list.append(9);
        let mut list2 = LinkedList::new();
        list2.append(0);
        list2.append(0);
        list2.append(2);

        let mut sum_list = LinkedList::new();
        sum_list.append(1);
        sum_list.append(0);
        sum_list.append(0);
        sum_list.append(1);

        assert_eq!(list.sum_lists_reversed(&list2), sum_list);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
