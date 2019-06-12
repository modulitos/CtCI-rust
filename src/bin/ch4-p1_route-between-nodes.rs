// Route Between Nodes: Given a directed graph, design an algorithm to
// find out whether there is a route between two nodes

use std::collections::HashSet;
use std::collections::VecDeque;

struct ASimpleGraph {
    adjacency_list: Vec<Vec<usize>>,
}

impl ASimpleGraph {
    fn new(list: Vec<Vec<usize>>) -> Self {
        ASimpleGraph {
            adjacency_list: list,
        }
    }

    fn visit(&self, from: usize, to: usize, visited: &mut HashSet<usize>) -> bool {
        visited.insert(from);
        if self.adjacency_list[from].contains(&to) {
            // The From node is directly connected to the To node
            true
        } else {
            // Check for a route in all of From's adjacent nodes:
            self.adjacency_list[from]
                .iter()
                .find_map(|neighbor| {
                    if !visited.contains(neighbor)  // prevent looping
                        && self.visit(*neighbor, to, visited)
                    {
                        Some(true)
                    } else {
                        None
                    }
                })
                .is_some()
        }
    }

    // DFS:
    fn route_between_nodes(&self, start: usize, end: usize) -> bool {
        let mut visited_nodes = HashSet::<usize>::new();
        self.visit(start, end, &mut visited_nodes)
    }

    fn route_between_nodes_bfs(&self, start: usize, end: usize) -> bool {
        let mut visited_nodes = HashSet::<usize>::new();
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_front(start);
        while !q.is_empty() {
            let next = q.pop_back().unwrap();
            if next == end {
                return true;
            }
            visited_nodes.insert(next);
            for neighbor in self.adjacency_list[next].iter() {
                if !visited_nodes.contains(neighbor) {
                    visited_nodes.insert(*neighbor);
                    q.push_front(*neighbor);
                }
            }
        }
        false
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_graph() {
        let _ = ASimpleGraph::new(vec![vec![1, 2, 3], vec![2], vec![0], vec![]]);
        assert!(true);
    }

    #[test]
    fn route_one_edge_between_nodes_dfs() {
        let g = ASimpleGraph::new(vec![vec![1, 3], vec![], vec![0], vec![]]);
        assert_eq!(g.route_between_nodes(0, 1), true);
        assert_eq!(g.route_between_nodes(0, 2), false);
        assert_eq!(g.route_between_nodes(0, 3), true);

        assert_eq!(g.route_between_nodes(2, 0), true);
        assert_eq!(g.route_between_nodes(1, 3), false);
    }

    #[test]
    fn route_one_edge_between_nodes_bfs() {
        let g = ASimpleGraph::new(vec![vec![1, 3], vec![], vec![0], vec![]]);
        assert_eq!(g.route_between_nodes_bfs(0, 1), true);
        assert_eq!(g.route_between_nodes_bfs(0, 2), false);
        assert_eq!(g.route_between_nodes_bfs(0, 3), true);

        assert_eq!(g.route_between_nodes_bfs(2, 0), true);
        assert_eq!(g.route_between_nodes_bfs(1, 3), false);
    }

    #[test]
    fn route_between_nodes_dfs() {
        let g = ASimpleGraph::new(vec![vec![3, 2], vec![], vec![0, 3], vec![1]]);
        assert_eq!(g.route_between_nodes(0, 1), true);
        assert_eq!(g.route_between_nodes(3, 0), false);
    }

    #[test]
    fn route_between_nodes_bfs() {
        let g = ASimpleGraph::new(vec![vec![3, 2], vec![], vec![0, 3], vec![1]]);
        assert_eq!(g.route_between_nodes_bfs(0, 1), true);
        assert_eq!(g.route_between_nodes_bfs(3, 0), false);
    }
}
