// Minimal Tree: Given a sorted (increasing order) array with unique
// integer elements, write an algorithm to create a binary search
// tree with minimal height.

use cracking::BinarySearchTree;
use std::collections::VecDeque;

trait MinimalTree<T> {
    fn insert_sorted_items(&mut self, sorted: Vec<T>);
}

// TODO: consider passing a callback here that adds the item directly,
// instead of building up an iterator.
fn re_sort_vec<T>(start: usize, end: usize, sorted: &Vec<T>) -> impl Iterator<Item = T>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    if start < end {
        let median_index = (start + end) / 2;
        let median = sorted[median_index].clone();
        println!("median: {:?}", median);
        let mut iter1 = re_sort_vec(start, median_index, sorted).into_iter();
        let mut iter2 = re_sort_vec(median_index + 1, end, sorted).into_iter();
        let mut result = VecDeque::<T>::new();
        result.push_back(median);
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(item1), Some(item2)) => {
                    result.push_back(item1);
                    result.push_back(item2);
                },
                (Some(item1), None) => result.push_back(item1),
                (None, Some(item2)) => result.push_back(item2),
                (None, None) => break,
            }
        }
        result.into_iter()
    } else {
        if start != end {
            panic!("end is less than start?! start:{}, end: {}", start, end);
        }
        // Base case - return an empty iterator:
        VecDeque::<T>::new().into_iter()
    }
}

impl<T> MinimalTree<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    // TODO: could be more perfomant by recursively adding nodes to
    // the BST directly, instead of going through the root node every
    // time with the `self.add` method
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
