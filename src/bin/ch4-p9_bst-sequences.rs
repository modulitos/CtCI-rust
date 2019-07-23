// BST Sequences: A binary search tree was created by traversing
// through an array from left to right and inserting each element.
// Given a binary search tree with distinct elements, print all
// possible arrays that could have led to this tree.

// EXAMPLE Input:
//    2
//   / \
//  1   3

// Output: {2, 1, 3}, {2, 3, 1}

// merged arrays:
// { 2, { 1}, {3 }

// Clarifying questions:
// assume no duplicates?

use cracking::{BinarySearchTree, Tree};
use std::collections::HashSet;
use std::iter::FromIterator;

fn get_all_arrays<T>(bst: BinarySearchTree<T>) -> HashSet<Vec<T>>
where
    T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug,
{
    get_all_arrays_rec(&bst.root)
}

fn get_all_arrays_rec<T>(node: &Tree<T>) -> HashSet<Vec<T>>
where
    T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug,
{
    if let Some(n) = &node {
        let node_value: T = n.data;
        // Get all the arrays for the left and right children:
        let left_arrays = get_all_arrays_rec(&n.left);
        let right_arrays = get_all_arrays_rec(&n.right);

        // "weave" all combinations of the l/r arrays:
        let mut results = HashSet::<Vec<T>>::new();
        for left_array in left_arrays {
            for right_array in right_arrays.clone() {
                let weaved: HashSet<Vec<T>> =
                    weave_arrays(left_array.clone(), right_array, vec![node_value]);
                results = results.union(&weaved).cloned().collect();
            }
        }
        results
    } else {
        [vec![]].iter().cloned().collect()
    }
}

// return an array containing every possible combination of both
// arrays, retaining the order of each array:

// We are merging two arrays in all possible ways, while keeping the
// elements within each array in the same relative order.
fn weave_arrays<T>(left: Vec<T>, right: Vec<T>, prefix: Vec<T>) -> HashSet<Vec<T>>
where
    T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug,
{
    // Base Case: If one of the arrays is empty:
    if left.len() == 0 {
        let mut new_prefix = prefix.clone();
        new_prefix.append(&mut right.clone());
        let mut results = HashSet::<Vec<T>>::new();
        results.insert(new_prefix);
        results
    } else if right.len() == 0 {
        let mut new_prefix = prefix.clone();
        new_prefix.append(&mut left.clone());
        let mut results = HashSet::<Vec<T>>::new();
        results.insert(new_prefix);
        results
    } else {
        // recursive case:
        let mut new_left = left.clone();
        let mut left_prefix = prefix.clone();
        left_prefix.push(new_left.remove(0));
        let left_weaves = weave_arrays(new_left, right.clone(), left_prefix);

        let mut new_right = right.clone();
        let mut right_prefix = prefix.clone();
        right_prefix.push(new_right.remove(0));
        let right_weaves = weave_arrays(left.clone(), new_right, right_prefix);

        left_weaves.union(&right_weaves).cloned().collect()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_weave_arrays_empty() {
        let left = vec![];
        let right = vec![];
        assert_eq!(
            weave_arrays(left.clone(), right.clone(), vec![3]),
            [vec![3]].iter().cloned().collect()
        );
    }

    #[test]
    fn test_weave_arrays_one_side_empty() {
        let left = vec![];
        let right = vec![1];
        assert_eq!(
            weave_arrays(left.clone(), right.clone(), vec![2]),
            [vec![2, 1]].iter().cloned().collect()
        );
    }

    #[test]
    fn test_weave_arrays_simple() {
        let left = vec![1];
        let right = vec![3];
        assert_eq!(
            weave_arrays(left.clone(), right.clone(), vec![]),
            [vec![1, 3], vec![3, 1]].iter().cloned().collect()
        );
        // ensure that ordering does not matter:
        assert_eq!(
            weave_arrays(left, right, vec![]),
            [vec![3, 1], vec![1, 3]].iter().cloned().collect()
        );
    }

    #[test]
    fn test_weave_arrays_varying_lengths() {
        let left = vec![1];
        let right = vec![2, 3, 4]; // right is longer than left
        assert_eq!(
            weave_arrays(left.clone(), right.clone(), vec![]),
            [
                vec![1, 2, 3, 4],
                vec![2, 1, 3, 4],
                vec![2, 3, 1, 4],
                vec![2, 3, 4, 1]
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_weave_arrays_simple_prefix() {
        let left = vec![1];
        let right = vec![3];
        assert_eq!(
            weave_arrays(left.clone(), right.clone(), vec![9]),
            [vec![9, 1, 3], vec![9, 3, 1]].iter().cloned().collect()
        );
    }

    #[test]
    fn test_weave_arrays_complex() {
        let left = vec![4, 5];
        let right = vec![1, 2, 0];
        assert_eq!(
            weave_arrays(left, right, vec![3]),
            [
                vec![3, 4, 5, 1, 2, 0],
                vec![3, 4, 1, 5, 2, 0],
                vec![3, 4, 1, 2, 5, 0],
                vec![3, 4, 1, 2, 0, 5],
                vec![3, 1, 2, 0, 4, 5],
                vec![3, 1, 2, 4, 0, 5],
                vec![3, 1, 2, 4, 5, 0],
                vec![3, 1, 4, 2, 5, 0],
                vec![3, 1, 4, 2, 0, 5],
                vec![3, 1, 4, 5, 2, 0],
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn get_all_arrays_simple() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        bst.add(1);
        bst.add(3);

        assert_eq!(
            get_all_arrays(bst),
            [vec![2, 1, 3], vec![2, 3, 1]].iter().cloned().collect()
        );
    }

    #[test]
    fn get_all_arrays_complex() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(3);
        bst.add(1);
        bst.add(0);
        bst.add(2);
        bst.add(4);
        bst.add(5);

        assert_eq!(
            get_all_arrays(bst),
            [
                // weaving: [4, 5], [1,0,2]
                vec![3, 4, 5, 1, 0, 2],
                vec![3, 4, 1, 5, 0, 2],
                vec![3, 4, 1, 0, 2, 5],
                vec![3, 4, 1, 0, 5, 2],
                vec![3, 1, 0, 2, 4, 5],
                vec![3, 1, 0, 4, 2, 5],
                vec![3, 1, 0, 4, 5, 2],
                vec![3, 1, 4, 0, 5, 2],
                vec![3, 1, 4, 0, 2, 5],
                vec![3, 1, 4, 5, 0, 2],
                // weaving: [4, 5], [1,2,0]
                vec![3, 4, 5, 1, 2, 0],
                vec![3, 4, 1, 5, 2, 0],
                vec![3, 4, 1, 2, 5, 0],
                vec![3, 4, 1, 2, 0, 5],
                vec![3, 1, 2, 0, 4, 5],
                vec![3, 1, 2, 4, 0, 5],
                vec![3, 1, 2, 4, 5, 0],
                vec![3, 1, 4, 2, 5, 0],
                vec![3, 1, 4, 2, 0, 5],
                vec![3, 1, 4, 5, 2, 0],
            ]
            .iter()
            .cloned()
            .collect()
        );
    }
}
