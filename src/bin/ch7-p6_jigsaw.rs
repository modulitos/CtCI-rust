// Jigsaw: Implement an NxN jigsaw puzzle. Design the data structures
// and explain an algorithm to solve the puzzle. You can assume that
// you have a fits_with method which, when passed two puzzle edges,
// returns true if the two edges belong together.

use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Orientation {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Orientation {
    fn rotate_clockwise(self) -> Self {
        use Orientation::*;
        match self {
            TOP => RIGHT,
            RIGHT => BOTTOM,
            BOTTOM => LEFT,
            LEFT => TOP,
        }
    }

    fn rotate_counterclockwise(self) -> Self {
        use Orientation::*;
        match self {
            TOP => LEFT,
            LEFT => BOTTOM,
            BOTTOM => RIGHT,
            RIGHT => TOP,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Shape {
    IN,
    OUT,
    FLAT,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    HORIZONTAL,
    VERTICAL,
}

type EdgeId = (u8, u8, Direction);

#[derive(Debug)]
struct Edge {
    id: EdgeId,
    shape: Shape,
}

impl Edge {
    fn new(id: EdgeId, shape: Shape) -> Self {
        Edge { id, shape }
    }
}
struct Piece {
    edges: HashMap<Orientation, Edge>,
}

impl Piece {
    fn new(row: u8, col: u8, rows: u8, cols: u8) -> Self {
        let mut edges = HashMap::new();
        use Direction::*;
        edges.insert(
            Orientation::LEFT,
            Edge::new(
                (row, col, HORIZONTAL),
                if col == 0 { Shape::FLAT } else { Shape::OUT },
            ),
        );
        edges.insert(
            Orientation::TOP,
            Edge::new(
                (row, col, VERTICAL),
                if row == 0 { Shape::FLAT } else { Shape::OUT },
            ),
        );
        edges.insert(
            Orientation::RIGHT,
            Edge::new(
                (row, col + 1, HORIZONTAL),
                if col == cols - 1 {
                    Shape::FLAT
                } else {
                    Shape::IN
                },
            ),
        );
        edges.insert(
            Orientation::BOTTOM,
            Edge::new(
                (row + 1, col, VERTICAL),
                if row == rows - 1 {
                    Shape::FLAT
                } else {
                    Shape::IN
                },
            ),
        );
        Piece { edges }
    }

    fn rotate_by(&mut self, mut i: u8) {
        i = i % 4;
        // rotate the piece clockwise on each iteration:
        for _ in 0..i {
            let new_top = self.edges.remove(&Orientation::LEFT).expect("no LEFT");
            let new_right = self.edges.remove(&Orientation::TOP).expect("no TOP");
            self.edges.insert(Orientation::TOP, new_top);
            let new_bottom = self.edges.remove(&Orientation::RIGHT).expect("no RIGHTJjk");
            self.edges.insert(Orientation::RIGHT, new_right);
            let new_left = self.edges.remove(&Orientation::BOTTOM).expect("no BOTTOM");
            self.edges.insert(Orientation::BOTTOM, new_bottom);
            self.edges.insert(Orientation::LEFT, new_left);
        }
    }

    fn get_edge(&self, orientation: Orientation) -> &Edge {
        self.edges
            .get(&orientation)
            .expect("This piece is missing an edge!!!!")
    }

    fn is_corner(&self) -> bool {
        let left_right_flats = self
            .edges
            .iter()
            .filter(|(orientation, edge)| {
                edge.shape == Shape::FLAT
                    && [Orientation::LEFT, Orientation::RIGHT].contains(&orientation)
            })
            .count();
        let top_bottom_flats = self
            .edges
            .iter()
            .filter(|(orientation, edge)| {
                edge.shape == Shape::FLAT
                    && [Orientation::TOP, Orientation::BOTTOM].contains(&orientation)
            })
            .count();
        left_right_flats > 0 && top_bottom_flats > 0
    }
}

struct Puzzle {
    pieces: Vec<Vec<Piece>>,
}

impl Puzzle {
    fn new(rows: u8, cols: u8) -> Self {
        let pieces = (0..rows)
            .map(|row| {
                (0..cols)
                    .map(|col| Piece::new(row, col, rows, cols))
                    .collect()
            })
            .collect();
        Puzzle { pieces }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        // randomly re-orient the pieces:
        self.pieces.shuffle(&mut rng);

        let between = Uniform::from(0..4);
        let mut rng = rand::thread_rng();
        between.sample(&mut rng);
        self.pieces.iter_mut().for_each(|pieces_row| {
            pieces_row
                .iter_mut()
                .for_each(|piece| piece.rotate_by(between.sample(&mut rng)))
        });
    }

    fn get_piece(&self, row: usize, col: usize) -> &Piece {
        &self.pieces[row][col]
        // We can optionally use the `get` method if we're not sure if 'row/col' will be in bounds:

        // fn get_piece(&self, row_i: usize, col_i: usize) -> &Piece {
        // if let Some(row) = self.pieces.get(usize::from(row_i)) {
        //     row.get(usize::from(col_i))
        // } else {
        //     None
        // }
    }
}

#[test]
fn create_puzzle() {
    let mut p = Puzzle::new(2, 3);
    assert_eq!(p.get_piece(0, 0).is_corner(), true);
    assert_eq!(p.get_piece(0, 1).is_corner(), false);
    assert_eq!(p.get_piece(1, 1).is_corner(), false);
    assert_eq!(p.get_piece(1, 2).is_corner(), true);
}

#[test]
fn test_orientation() {
    let mut o = Orientation::LEFT;
    assert_eq!(o, Orientation::LEFT);
    o = o.rotate_clockwise();
    assert_eq!(o, Orientation::TOP);
    o = o.rotate_clockwise();
    o = o.rotate_clockwise();
    assert_eq!(o, Orientation::BOTTOM);
}

#[test]
fn test_piece() {
    use Orientation::*;
    let mut p = Piece::new(0, 0, 3, 3);
    let top = p.get_edge(TOP).id;
    let right = p.get_edge(RIGHT).id;
    let bottom = p.get_edge(BOTTOM).id;
    let left = p.get_edge(LEFT).id;
    assert_ne!(p.get_edge(TOP).id, left);
    assert_ne!(p.get_edge(RIGHT).id, top);
    assert_ne!(p.get_edge(BOTTOM).id, right);
    assert_ne!(p.get_edge(LEFT).id, bottom);
    p.rotate_by(1);
    assert_eq!(p.get_edge(TOP).id, left);
    assert_eq!(p.get_edge(RIGHT).id, top);
    assert_eq!(p.get_edge(BOTTOM).id, right);
    assert_eq!(p.get_edge(LEFT).id, bottom);
    p.rotate_by(2);
    assert_eq!(p.get_edge(BOTTOM).id, left);
    assert_eq!(p.get_edge(LEFT).id, top);
    assert_eq!(p.get_edge(TOP).id, right);
    assert_eq!(p.get_edge(RIGHT).id, bottom);
}
