// Build Order: You are given a list of projects and a list of
// dependencies (which is a list of pairs of projects, where the
// second project is dependent on the first project). All of a
// project's dependencies must be built before the project is. Find a
// build order that will allow the projects to be built. If there is
// no valid build order, return an error.

// EXAMPLE Input: projects: a, b, c, d, e, f
// dependencies: (a, d), (f, b), (b, d), (f, a), (d, c)

// Output: f, e, a, b, d, c

// clarifying questions:
// - does the order of the deps matter?

use cracking::{Graph, IntoChar};
use std::collections::HashSet;
use std::iter::FromIterator;

trait BuildOrder {
    fn get_order(&self) -> Option<Vec<char>>;
    fn _get_outgoing_nodes_for_node(&self, node: char) -> HashSet<char>;
}

impl BuildOrder for Graph {
    fn get_order(&self) -> Option<Vec<char>> {
        // Iterate each node, then check for all nodes with no deps
        // and add those nodes to our "built" list. If all of the nodes
        // that a node points to are in our built list, then that node
        // can be built as well. Continue iterating each node until
        // there are none left, or the "built" list size stops growing
        // - meaning there is no build order.

        let mut built: Vec<char> = vec![];

        while self.nodes() > built.len() {
            // get all nodes that are ready to be built:
            let mut ready_nodes: Vec<char> = self
                .nodes
                .iter()
                .filter(|node| {
                    // filter out nodes that are already built:
                    !built.contains(&node.into_char())
                })
                .filter_map(|node| {
                    // if all of the node's deps are built, then add that
                    // node to the built nodes:
                    let node_deps = self._get_outgoing_nodes_for_node(node.into_char());
                    // find a node that has all outgoing edges in our built list:
                    if node_deps.iter().all(|dep| built.contains(dep)) {
                        Some(node.into_char())
                    } else {
                        None
                    }
                })
                .collect();
            if ready_nodes.len() == 0 {
                return None;
            } else {
                println!("addying ready_nodes: {:?}", ready_nodes);
                built.append(&mut ready_nodes);
            }
        }
        Some(built)
    }

    fn _get_outgoing_nodes_for_node(&self, node: char) -> HashSet<char> {
        HashSet::from_iter(
            self.get_outgoing_edges_for_node(node)
                .into_iter()
                .map(|edge| edge.node.into_char()), // node is stored internally as a u64
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

    #[test]
    fn build_order_simple() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c']);
        g.set_edges('a', vec!['b']);
        g.set_edges('b', vec!['c']);
        assert_eq!(g.get_order(), Some(vec!['c', 'b', 'a']))
    }

    #[test]
    fn build_order_complex() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c', 'd', 'e', 'f']);
        g.set_edges('d', vec!['a']);
        g.set_edges('b', vec!['f']);
        g.set_edges('d', vec!['b']);
        g.set_edges('a', vec!['f']);
        g.set_edges('c', vec!['d']);
        assert_eq!(g.get_order(), Some(vec!['e', 'f', 'a', 'b', 'd', 'c']))
    }

    #[test]
    fn build_order_impossible() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c', 'd', 'e', 'f']);
        g.set_edges('d', vec!['a']);
        g.set_edges('b', vec!['f']);
        g.set_edges('d', vec!['b']);
        g.set_edges('a', vec!['f']);
        g.set_edges('f', vec!['d']);
        g.set_edges('c', vec!['d']);
        assert_eq!(g.get_order(), None)
    }
}
