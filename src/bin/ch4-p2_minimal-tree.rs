// Minimal Tree: Given a sorted (increasing order) array with unique
// integer elements, write an algorithm to create a binary search
// tree with minimal height.

use cracking::BinarySearchTree;
use std::collections::VecDeque;

trait MinimalTree<T> {
    fn insert_sorted_items(&mut self, sorted: Vec<T>);
}

fn re_sort_vec<T>(start: usize, end: usize, sorted: &Vec<T>) -> impl Iterator<Item = T>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    if start < end {
        let median_index = (start + end) / 2;
        let median = sorted[median_index].clone();
        println!("median: {:?}", median);
        let mut iter1 = re_sort_vec(start, median_index, sorted).into_iter().peekable();
        let mut iter2 = re_sort_vec(median_index + 1, end, sorted).into_iter().peekable();
        let mut result = VecDeque::<T>::new();
        result.push_back(median);
        while iter1.peek().is_some() || iter2.peek().is_some() {
            if iter1.peek().is_some() {
                result.push_back(iter1.next().unwrap());
            }
            if iter2.peek().is_some() {
                result.push_back(iter2.next().unwrap());
            }
        }
        result.into_iter()
    } else if start == end {
        // Base case - return an empty iterator:
        VecDeque::<T>::new().into_iter()
    } else {
        panic!("end is less than start?! start:{}, end: {}", start, end)
    }
}

impl<T> MinimalTree<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn insert_sorted_items(&mut self, sorted: Vec<T>) {
        // re-sort our Vec so that the median of the array comes
        // first, followed by the medians of each subarray:
        let resorted = re_sort_vec(0, sorted.len(), &sorted);
        resorted.for_each(|item| {
            println!("adding item: {:?}", item);
            self.add(item);
        });
    }
}

mod tests {
    use super::*;

    #[test]
    fn insert_sorted_items() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.insert_sorted_items(vec![1, 2, 3, 4, 5]);
        assert!(true);
    }

    #[test]
    fn insert_sorted_items_height_0() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.insert_sorted_items(vec![]);
        assert_eq!(bst.get_height(), 0);
    }

    #[test]
    fn insert_sorted_items_height_1() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.insert_sorted_items(vec![1]);
        assert_eq!(bst.get_height(), 1);
    }

    #[test]
    fn insert_sorted_items_height_2() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.insert_sorted_items(vec![1, 2, 3]);
        assert_eq!(bst.get_height(), 2);
    }

    #[test]
    fn insert_sorted_items_height_4() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.insert_sorted_items(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(bst.get_height(), 4);
    }
}
