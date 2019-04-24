use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;

// Inspired by:
// https://raw.githubusercontent.com/brndnmtthws/cracking-the-coding-interview-rust/master/src/bin/c02p01.rs
// but with some improvements:

//  * implements Eq and PartialEq traits, instead of a
//  "has_duplicates" function.
//  * uses an _unlink_node method on LinkedList, instead of on the
//  Node struct, which is more consistent with the official library

extern crate cracking;
use cracking::LinkedList;

pub trait RemoveDups {
    fn remove_duplicates(&mut self);
}

impl<T> RemoveDups for LinkedList<T>
where
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    fn remove_duplicates(&mut self) {
        let mut set: HashSet<u64> = HashSet::new();
        for node in self.iter() {
            let mut s = DefaultHasher::new();
            {
                let data = &node.borrow().data;
                data.hash(&mut s);
            }
            let hash = s.finish();
            if set.contains(&hash) {
                self.unlink_node(Some(node));
            }
            set.insert(hash);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_remove_duplicates() {
        let mut list1 = LinkedList::<String>::new();
        list1.append(String::from("item1"));
        list1.append(String::from("item2"));

        list1.remove_duplicates();
        let mut list2 = LinkedList::<String>::new();
        list2.append(String::from("item1"));
        list2.append(String::from("item2"));
        assert_eq!(list1, list2);

        list1.append(String::from("item1"));
        list1.remove_duplicates();

        assert_eq!(list1, list2);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    list.remove_duplicates();
}
