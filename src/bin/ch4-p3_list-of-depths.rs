// List of Depths: Given a binary tree, design an algorithm which
// creates a linked list of all the nodes at each depth (e.g., if you
// have a tree with depth 0, you'll have 0 linked lists).

use cracking::{BinarySearchTree, Tree};

trait ListOfDepths<T> {
    fn get_lists(&self) -> Vec<Vec<T>>;
    fn walk_lists(&self, node: &Tree<T>, height: usize, lists: &mut Vec<Vec<T>>) -> usize;
}

impl<T> ListOfDepths<T> for BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    fn get_lists(&self) -> Vec<Vec<T>> {
        let mut lists: Vec<Vec<T>> = vec![];
        self.walk_lists(&self.root, 0, &mut lists);
        lists
    }

    fn walk_lists(&self, node: &Tree<T>, height: usize, lists: &mut Vec<Vec<T>>) -> usize {
        println!("visiting node: {:?}", node);
        if let Some(n) = node {
            if height == lists.len() {
                lists.push(vec![]);
            }
            let l_height = self.walk_lists(&n.left, height + 1, lists);
            // add items to lists in order:
            lists[height].push(n.data.clone());
            let r_height = self.walk_lists(&n.right, height + 1, lists);
            std::cmp::max(l_height, r_height)
        } else {
            height
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn get_lists_empty() {
        let bst = BinarySearchTree::<u32>::new();
        bst.get_lists();
    }

    #[test]
    fn get_lists_1() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(1);
        let lists = bst.get_lists();
        assert_eq!(lists, vec![vec![1]]);
    }

    #[test]
    fn get_lists_3() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        bst.add(1);
        bst.add(3);
        let lists = bst.get_lists();
        assert_eq!(lists, vec![vec![2], vec![1, 3]]);
    }

    #[test]
    fn get_lists_7() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(4);
        bst.add(2);
        bst.add(6);
        bst.add(3);
        bst.add(1);
        bst.add(5);
        bst.add(7);
        let lists = bst.get_lists();
        assert_eq!(lists, vec![vec![4], vec![2, 6], vec![1, 3, 5, 7]]);
    }

    #[test]
    fn get_lists_8() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(4);
        bst.add(2);
        bst.add(6);
        bst.add(3);
        bst.add(1);
        bst.add(5);
        bst.add(7);
        bst.add(8);
        let lists = bst.get_lists();
        assert_eq!(lists, vec![vec![4], vec![2, 6], vec![1, 3, 5, 7], vec![8]]);
    }
}
