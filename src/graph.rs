use std::cmp::{Ord, Ordering};
use std::convert::TryFrom;

// largely inspired by:
// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/e79494a07c8d771e0d357ed05eb6d7ddb58a3bf8/Chapter05/src/graph.rs

pub type KeyType = u64;

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
pub struct Edge {
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
pub trait IntoEdge {
    fn into_edge(self) -> Edge;
}

impl IntoEdge for (u32, KeyType) {
    fn into_edge(self) -> Edge {
        Edge {
            weight: self.0,
            node: usize::try_from(self.1).unwrap(),
        }
    }
}
impl IntoEdge for (u32, char) {
    fn into_edge(self) -> Edge {
        Edge {
            weight: self.0,
            node: usize::try_from(u32::from(self.1)).unwrap(),
        }
    }
}
impl IntoEdge for char {
    fn into_edge(self) -> Edge {
        Edge {
            weight: 0,
            node: usize::try_from(u32::from(self)).unwrap(),
        }
    }
}

impl IntoEdge for KeyType {
    fn into_edge(self) -> Edge {
        Edge {
            weight: 0,
            node: usize::try_from(self).unwrap(),
        }
    }
}

pub trait IntoNode {
    fn into_node(self) -> KeyType;
}
impl IntoNode for KeyType {
    fn into_node(self) -> KeyType {
        self
    }
}

impl IntoNode for char {
    fn into_node(self) -> KeyType {
        KeyType::from(u32::from(self))
    }
}
impl IntoNode for (u32, u64) {
    fn into_node(self) -> KeyType {
        self.1
    }
}
impl IntoNode for (u32, char) {
    fn into_node(self) -> KeyType {
        KeyType::from(u32::from(self.1))
    }
}

pub trait IntoNodes {
    fn into_nodes(self) -> Vec<KeyType>;
}

impl IntoNodes for Vec<char> {
    fn into_nodes(self) -> Vec<KeyType> {
        self.iter()
            .map(|c| KeyType::from(u32::from(c.clone())))
            .collect()
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

    pub fn edges(&self) -> u64 {
        self.adjacency_list
            .iter()
            .fold(0u64, |acc, node_edges| acc + node_edges.len() as u64)
    }

    pub fn nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn set_nodes<I>(&mut self, nodes: I)
    where
        I: IntoNodes,
    {
        self.nodes = nodes.into_nodes();
        self.adjacency_list = vec![vec![]; self.nodes.len()]
    }

    pub fn set_edges<T>(&mut self, from: impl IntoNode, edges: Vec<T>)
    where
        T: IntoEdge + IntoNode + std::clone::Clone,
    {
        let from = from.into_node();
        let edges: Vec<Edge> = edges
            .into_iter()
            .filter_map(|e| {
                if let Some(_) = self.nodes.iter().position(|n| n == &e.clone().into_node()) {
                    Some(e.into_edge())
                } else {
                    panic!("Node does not exist");
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

    #[test]
    fn insert_edges() {
        let mut g = Graph::new();
        g.set_nodes(vec![1, 2, 3]);
        g.set_edges(1, vec![(0, 1), (0, 2), (0, 3)]);
        g.set_edges(2, vec![(0, 3)]);
        assert_eq!(g.nodes(), 3);
        assert_eq!(g.edges(), 4);
    }

    #[test]
    #[should_panic(expected = "Node does not exist")]
    fn insert_edges_panic() {
        let mut g = Graph::new();
        g.set_nodes(vec![1, 2, 3]);
        g.set_edges(1, vec![(0, 1), (0, 2), (0, 3)]);
        g.set_edges(2, vec![(0, 5)]);
        assert_eq!(g.nodes(), 3);
        assert_eq!(g.edges(), 3);
    }

    #[test]
    fn insert_edges_as_char() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c']);
        g.set_edges('a', vec![(1, 'a'), (2, 'b'), (3, 'c')]);
        g.set_edges('b', vec![(7, 'a')]);
        assert_eq!(g.nodes(), 3);
        assert_eq!(g.edges(), 4);
    }

    #[test]
    fn insert_edges_no_weight() {
        let mut g = Graph::new();
        g.set_nodes(vec![1, 2, 3]);
        g.set_edges(1, vec![1, 2, 3]);
        g.set_edges(2, vec![3]);
        assert_eq!(g.nodes(), 3);
        assert_eq!(g.edges(), 4);
    }

    #[test]
    fn insert_edges_chars_no_weight() {
        let mut g = Graph::new();
        g.set_nodes(vec!['a', 'b', 'c']);
        g.set_edges('a', vec!['a', 'b', 'c']);
        g.set_edges('b', vec!['a']);
        assert_eq!(g.nodes(), 3);
        assert_eq!(g.edges(), 4);
    }
}
