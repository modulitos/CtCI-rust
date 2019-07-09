// Build Order: You are given a list of projects and a list of
// dependencies (which is a list of pairs of projects, where the
// second project is dependent on the first project). All of a
// project's dependencies must be built before the project is. Find a
// build order that will allow the projects to be built. If there is
// no valid build order, return an error.

// EXAMPLE Input: projects: a, b, c, d, e, f
// dependencies: (a, d), (f, b), (b, d), (f, a), (d, c)

// Output: f, e, a, b, d, c

use cracking::{Graph, KeyType};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::convert::TryFrom;

trait BuildOrder {
    fn get_order(&self) -> Option<HashSet<char>>;
    fn _get_outgoing_nodes_for_node(&self, node: char) -> HashSet<char>;
}

impl BuildOrder for Graph {
    fn get_order(&self) -> Option<HashSet<char>> {
        // Iterate each node, then check for all nodes with no deps
        // and add those nodes to our "built" list. If all of the nodes
        // that a node points to are in our built list, then that node
        // can be built as well. Continue iterating each node until
        // there are none left, or the "built" list size stops growing
        // - meaning there is no build order.
        None
    }

    fn _get_outgoing_nodes_for_node(&self, node: char) -> HashSet<char> {
        HashSet::from_iter(
            self.get_outgoing_edges_for_node(node)
                .into_iter()
                .map(|edge| char::from(u8::try_from(edge.node).unwrap())), // node is stored internally as a u64
        )
    }
}

mod tests {
    use super::*;

    #[test]
    fn get_outgoing_nodes_for_node() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c']);
        g.set_edges('a', vec!['a', 'b', 'c']);
        g.set_edges('b', vec!['a']);
        assert_eq!(
            g._get_outgoing_nodes_for_node('b'),
            HashSet::from_iter(vec!['a'].into_iter())
        );
        assert_eq!(
            g._get_outgoing_nodes_for_node('a'),
            HashSet::from_iter(vec!['a', 'b', 'c'].into_iter())
        );
        assert_eq!(
            g._get_outgoing_nodes_for_node('c'),
            HashSet::from_iter(vec![].into_iter())
        );
        assert_ne!(
            g._get_outgoing_nodes_for_node('b'),
            HashSet::from_iter(vec!['a', 'b', 'c'].into_iter())
        );
    }
}
