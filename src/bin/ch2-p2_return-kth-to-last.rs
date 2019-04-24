extern crate cracking;
use cracking::LinkedList;

pub trait RemoveKToLast<T> {
    fn kth_to_last(&self, k: usize) -> T;
}

impl<T> RemoveKToLast<T> for LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    fn kth_to_last(&self, k: usize) -> T {
        // use "tortoise/hare" method:
        let mut kth_last_node = None; // our 'tortoise'
        for (n, _) in self.iter().enumerate() {
            // _ is the hare
            if n == k {
                kth_last_node = self.head.clone();
            } else if n > k {
                kth_last_node = kth_last_node.unwrap().borrow().next.clone();
            }
        }
        kth_last_node.unwrap().borrow().data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kth_node() {
        let mut list = LinkedList::<usize>::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);

        assert_eq!(list.kth_to_last(1), 3);
        assert_eq!(list.kth_to_last(2), 2);
        assert_eq!(list.kth_to_last(0), 4);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
}
