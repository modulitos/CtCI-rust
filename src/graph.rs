use std::cmp::{Ord, Ordering};
// use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::iter::FromIterator;

// largely inspired by:
// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/e79494a07c8d771e0d357ed05eb6d7ddb58a3bf8/Chapter05/src/graph.rs

type KeyType = u64;

#[derive(Eq, PartialEq, Clone, Debug)]
enum TentativeWeight {
    Infinite,
    Number(u32),
}

impl Ord for TentativeWeight {
    fn cmp(&self, other: &TentativeWeight) -> Ordering {
        match other {
            TentativeWeight::Infinite => match self {
                TentativeWeight::Infinite => Ordering::Equal,
                _ => Ordering::Less,
            },
            TentativeWeight::Number(o) => match self {
                TentativeWeight::Infinite => Ordering::Greater,
                TentativeWeight::Number(s) => s.cmp(o),
            },
        }
    }
}

impl PartialOrd for TentativeWeight {
    fn partial_cmp(&self, other: &TentativeWeight) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Edge {
    weight: u32,
    node: usize,
}

fn min_index(weights: &Vec<TentativeWeight>, nodes: &Vec<usize>) -> usize {
    let mut min_weight = (weights[0].clone(), 0);
    for node in nodes.iter() {
        if let Some(n) = weights.get(*node) {
            if n < &min_weight.0 {
                min_weight = ((&weights[*node]).clone(), node.clone())
            }
        }
    }
    return min_weight.1;
}

// https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17
trait IntoEdges {
    fn into_edges(self) -> Vec<Edge>;
}

pub trait IntoNodes {
    fn into_nodes(self) -> Vec<KeyType>;
}

impl IntoNodes for Vec<char> {
    fn into_nodes(self) -> Vec<KeyType> {
        self.iter().map(|c| KeyType::from(u32::from(c.clone()))).collect()
    }
}
impl IntoNodes for Vec<u64> {
    fn into_nodes(self) -> Vec<KeyType> {
        self
    }
}

pub struct Graph {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<KeyType>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adjacency_list: vec![],
            nodes: vec![],
        }
    }

    fn get_node_index(&self, node: KeyType) -> Option<usize> {
        self.nodes.iter().position(|n| n == &node)
    }

    pub fn edges(&self) -> u64 {
        self.adjacency_list
            .iter()
            .fold(0u64, |acc, node_edges| acc + node_edges.len() as u64)
    }

    pub fn nodes(&self) -> usize {
        self.nodes.len()
    }

    // pub fn set_nodes(&mut self, nodes: Vec<KeyType>) {
    pub fn set_nodes<I>(&mut self, nodes: I)
    where
        I: IntoNodes,
    {
        self.nodes = nodes.into_nodes();
        self.adjacency_list = vec![vec![]; self.nodes.len()]
    }

    pub fn set_edges(&mut self, from: KeyType, edges: Vec<(u32, KeyType)>) {
        let edges: Vec<Edge> = edges
            .into_iter()
            .filter_map(|e| {
                if let Some(to) = self.get_node_index(e.1) {
                    Some(Edge {
                        weight: e.0,
                        node: to,
                    })
                } else {
                    println!("node not available! {:?}", e.1);
                    None
                }
            })
            .collect();
        match self.nodes.iter().position(|n| n == &from) {
            Some(i) => self.adjacency_list[i] = edges,
            None => {
                self.nodes.push(from);
                self.adjacency_list.push(edges)
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_graph() {
        let _ = Graph::new();
        assert!(true);
    }

    #[test]
    fn insert_nodes() {
        let mut g = Graph::new();
        g.set_nodes(vec![1, 2, 3]);
        assert_eq!(g.nodes(), 3);
    }

    #[test]
    fn insert_nodes_as_chars() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c']);
        assert_eq!(g.nodes(), 3);
    }
}
