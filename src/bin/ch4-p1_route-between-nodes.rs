// Route Between Nodes: Given a directed graph, design an algorithm to
// find out whether there is a route between two nodes

use std::collections::HashSet;

struct ASimpleGraph {
    adjacency_list: Vec<Vec<usize>>,
}

impl ASimpleGraph {
    fn new(list: Vec<Vec<usize>>) -> Self {
        ASimpleGraph {
            adjacency_list: list,
        }
    }

    // fn visit(&self, from: usize, to: usize, visited: Rc<RefCell<HashSet<usize>>>) -> bool {
    fn visit(&self, from: usize, to: usize, visited: &mut HashSet<usize>) -> bool {
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
                        visited.insert(from);
                        Some(true)
                    } else {
                        None
                    }
                })
                .is_some()
        }
    }

    fn route_between_nodes(&self, n1: usize, n2: usize) -> bool {
        let mut visited_nodes = HashSet::<usize>::new();
        self.visit(n1, n2, &mut visited_nodes)
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
    fn route_one_edge_between_nodes() {
        let g = ASimpleGraph::new(vec![vec![1, 3], vec![], vec![0], vec![]]);
        assert_eq!(g.route_between_nodes(0, 1), true);
        assert_eq!(g.route_between_nodes(0, 2), false);
        assert_eq!(g.route_between_nodes(0, 3), true);

        assert_eq!(g.route_between_nodes(2, 0), true);
        assert_eq!(g.route_between_nodes(1, 3), false);
    }
}
