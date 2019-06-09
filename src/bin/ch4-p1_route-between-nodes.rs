// Route Between Nodes: Given a directed graph, design an algorithm to
// find out whether there is a route between two nodes

struct ASimpleGraph {
 adjacency_list: Vec<Vec<usize>>,
}

impl ASimpleGraph {
    fn new(list: Vec<Vec<usize>>) -> Self {
        ASimpleGraph {
            adjacency_list: list,
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn create_graph() {
        let _ = ASimpleGraph::new(vec![vec![1,2,3], vec![2], vec![0], vec![]]);
        assert!(true);
    }
}
