use std::mem;

// This implementation is largely inspired by:
// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/e79494a07c8d771e0d357ed05eb6d7ddb58a3bf8/Chapter05/src/binary_search_tree.rs
use std::cell::RefCell;
use std::rc::Rc;

type BareTree<T> = Rc<RefCell<Node<T>>>;
pub type Tree<T> = Option<BareTree<T>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
    pub parent: Tree<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Tree<T> {
        Some(Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
            parent: None,
        })))
    }

    // Using the builder pattern to allow for optional arguments.
    pub fn with_parent(mut self, parent: Node<T>) -> Self {
        self.parent = Some(Rc::new(RefCell::new(parent)));
        self
    }

    pub fn create(self) -> Self {
        self
    }

    pub fn new_node(data: T) -> Node<T> {
        Node {
            data,
            left: None,
            right: None,
            parent: None,
        }
    }

    // pub fn set_left(&mut self, mut node: Node<T>) {
    //     node.parent = self.clone();
    //     self.left = Some(Box::new(node));
    // }

    // pub fn set_right(&mut self, node: Node<T>) {
    //     node.parent = self;
    //     self.right = Some(Box::new(node));
    // }
}

pub struct BinarySearchTree<T> {
    pub root: Tree<T>,
    pub length: u64,
}

impl<T> BinarySearchTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }

    // // Using the builder pattern to allow for optional arguments.
    // pub fn with_root(mut self, root: Node<T>) -> Self {
    //     self.root = Some(Box::new(root));
    //     self
    // }

    // pub fn create(self) -> Self {
    //     self
    // }

    pub fn add(&mut self, data: T) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, data);
    }

    fn add_rec(&mut self, node: Tree<T>, data: T) -> Tree<T> {
        println!("visiting node: {:?}, with data: {:?}", node, data);
        match node {
            Some(n) => {
                {
                    let mut n_mut = n.borrow_mut();
                    if data <= n_mut.data {
                        n_mut.left = self.add_rec(n_mut.left.clone(), data);
                    } else {
                        n_mut.right = self.add_rec(n_mut.right.clone(), data);
                    }
                }
                Some(n)
            }
            _ => Node::new(data),
        }
    }

    pub fn find(&self, data: T) -> Option<T> {
        self.find_r(&self.root, data)
    }

    fn find_r(&self, node: &Tree<T>, data: T) -> Option<T> {
        match node {
            Some(n) => {
                if n.clone().borrow().data == data {
                    Some(n.borrow().data.clone())
                } else if data < n.borrow().data {
                    self.find_r(&n.clone().borrow().left, data)
                } else {
                    self.find_r(&n.clone().borrow().right, data)
                }
            }
            _ => None,
        }
    }

    pub fn get_height(&self) -> usize {
        // traverse the tree dfs, tracking the height along the way:
        self.walk_in_order(&self.root, 0)
    }

    fn walk_in_order(&self, node: &Tree<T>, height: usize) -> usize {
        println!("visiting node: {:?}", node);
        if let Some(n) = node {
            let l_height = self.walk_in_order(&n.clone().borrow().left, height + 1);
            let r_height = self.walk_in_order(&n.clone().borrow().right, height + 1);
            std::cmp::max(l_height, r_height)
        } else {
            height
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_bst() {
        let _ = BinarySearchTree::<u32>::new();
        assert!(true);
    }

    #[test]
    fn add_to_bst() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(1);
        bst.add(2);
        bst.add(3);
        bst.add(4);
        bst.add(5);
        bst.add(6);
        bst.add(7);
        assert!(true);
    }

    #[test]
    fn find_in_bst_random() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(5);
        bst.add(3);
        bst.add(9);
        bst.add(1);
        bst.add(6);
        bst.add(0);
        bst.add(7);
        bst.add(2);
        bst.add(8);
        assert_eq!(bst.find(10), None);
        assert_eq!(bst.find(0), Some(0));
        assert_eq!(bst.find(3), Some(3));
    }

    #[test]
    fn find_in_bst_incremental() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(0);
        bst.add(1);
        bst.add(2);
        bst.add(3);
        bst.add(4);
        bst.add(5);
        bst.add(6);
        bst.add(7);
        bst.add(8);
        bst.add(9);
        assert_eq!(bst.find(10), None);
        assert_eq!(bst.find(0), Some(0));
        assert_eq!(bst.find(3), Some(3));
    }

    #[test]
    fn test_get_height_0() {
        let bst = BinarySearchTree::<u32>::new();
        assert_eq!(bst.get_height(), 0);
    }
    #[test]
    fn test_get_height_1() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(1);
        assert_eq!(bst.get_height(), 1);
    }
    #[test]
    fn test_get_height_10() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(0);
        bst.add(1);
        bst.add(2);
        bst.add(3);
        bst.add(4);
        bst.add(5);
        bst.add(6);
        bst.add(7);
        bst.add(8);
        bst.add(9);
        assert_eq!(bst.get_height(), 10);
    }

    #[test]
    fn test_get_minimal_height_2() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(2);
        bst.add(1);
        bst.add(3);
        assert_eq!(bst.get_height(), 2);
    }

    #[test]
    fn test_get_minimal_height_3() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(5);
        bst.add(8);
        bst.add(2);
        bst.add(1);
        bst.add(3);
        bst.add(9);
        bst.add(7);
        assert_eq!(bst.get_height(), 3);
    }

    #[test]
    fn test_get_minimal_height_4() {
        let mut bst = BinarySearchTree::<u32>::new();
        bst.add(5);
        bst.add(8);
        bst.add(2);
        bst.add(1);
        bst.add(3);
        bst.add(9);
        bst.add(7);
        bst.add(6);
        assert_eq!(bst.get_height(), 4);
    }
}
