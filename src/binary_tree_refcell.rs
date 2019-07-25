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
    pub fn new(data: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        let new_node = Some(Rc::new(RefCell::new(Node {
            data,
            left: left.clone(),
            right: right.clone(),
            parent: None,
        })));
        if let Some(l) = left {
            l.borrow_mut().parent = new_node.clone();
        }
        if let Some(r) = right {
            r.borrow_mut().parent = new_node.clone();
        }
        new_node
    }
}

pub struct BinaryTree<T> {
    pub root: Tree<T>,
    pub length: u64,
}

impl<T> BinaryTree<T>
where
    T: std::cmp::PartialOrd + std::clone::Clone + std::fmt::Debug,
{
    pub fn new(node: Tree<T>) -> Self {
        BinaryTree {
            root: node,
            length: 0,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_simple_bt() {
        let node = Node::new(1, Node::new(2, None, None), None);
        BinaryTree::<u32>::new(node);
        assert!(true);
    }

    #[test]
    fn create_complex_bt() {
        let node = Node::new(
            1,
            Node::new(2, None, None),
            Node::new(3, Node::new(4, None, None), Node::new(5, None, None)),
        );
        BinaryTree::<u32>::new(node);
        assert!(true);
    }

    #[test]
    fn compare_node_parents() {
        let grandchild = Node::new(4, None, None);
        let child = Node::new(3, grandchild.clone(), Node::new(5, None, None));
        assert!(Rc::ptr_eq(
            &grandchild.as_ref().unwrap().borrow().parent.as_ref().unwrap(),
            &child.clone().unwrap()
        ));
        let node = Node::new(1, Node::new(2, None, None), child);
        BinaryTree::<u32>::new(node);
        assert!(true);
    }
}
