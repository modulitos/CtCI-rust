// Jigsaw: Implement an NxN jigsaw puzzle. Design the data structures
// and explain an algorithm to solve the puzzle. You can assume that
// you have a fits_with method which, when passed two puzzle edges,
// returns true if the two edges belong together.

use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum Shape {
    IN,
    OUT,
    FLAT,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    HORIZONTAL,
    VERTICAL,
}

type EdgeId = (usize, usize, Direction);

#[derive(Debug, Clone)]
struct Edge {
    id: EdgeId,
    shape: Shape,
}

impl Edge {
    fn new(id: EdgeId, shape: Shape) -> Self {
        Edge { id, shape }
    }

    fn fits_with(&self, other: &Edge) -> bool {
        self.id == other.id
    }
}
struct Piece {
    edges: HashMap<Orientation, Edge>,
}

impl Piece {
    fn new(row: usize, col: usize, rows: usize, cols: usize) -> Self {
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

    fn set_edge_as_orientation(&mut self, from: &Orientation, to: &Orientation) {
        // let i = (to - from + 4) % 4;
        // self.rotate_by(i);
    }
}

struct Puzzle {
    pieces: Vec<Piece>,
    rows: usize,
    cols: usize,
}

impl Puzzle {
    fn new(rows: usize, cols: usize) -> Self {
        Puzzle {
            pieces: (0..(rows * cols))
                .map(|val| Piece::new(val / cols, val % cols, rows, cols))
                .collect(),
            rows,
            cols,
        }
    }

    fn get_piece(&self, row: usize, col: usize) -> &Piece {
        // NOTE: We can optionally use the `get` method if we're not
        // sure if 'row/col' will be in bounds, which would return an
        // Option<&Piece>
        // &self.pieces[row][col]
        &self.pieces[(row * self.cols) + col]
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        // randomly re-orient the pieces:
        self.pieces.shuffle(&mut rng);

        let between = Uniform::from(0..4);
        let mut rng = rand::thread_rng();
        between.sample(&mut rng);
        // randomly rotate the pieces:
        self.pieces
            .iter_mut()
            .for_each(|piece| piece.rotate_by(between.sample(&mut rng)));
    }

    fn solve(&mut self) {
        use Orientation::*;
        // build our array of pieces back up again, with the correct
        // order and orientation:

        // Find the first piece to seed our solution:
        let top_left_i = self
            .pieces
            .iter_mut()
            .enumerate()
            .find_map(|(i, piece)| {
                if let Some(orientation) = piece.edges.iter().find_map(|(orientation, edge)| {
                    if edge.id == (0, 0, Direction::HORIZONTAL) {
                        Some(orientation.clone())
                    } else {
                        None
                    }
                }) {
                    piece.set_edge_as_orientation(&orientation, &LEFT);
                    Some(i)
                } else {
                    None
                }
            })
            .expect("no top left piece!");
        self.pieces.swap(0, top_left_i);
        // iterate through self.pieces, progressively searching for
        // the next piece that matches our existing piece
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
fn test_piece_orientation() {
    use Orientation::*;
    let mut p = Piece::new(0, 0, 3, 3);
    let top = p.get_edge(TOP).id.clone();
    let right = p.get_edge(RIGHT).id.clone();
    let bottom = p.get_edge(BOTTOM).id.clone();
    let left = p.get_edge(LEFT).id.clone();
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

#[test]
fn test_piece_fits_with() {
    let p = Puzzle::new(2, 3);
    use Orientation::*;
    assert_eq!(
        p.get_piece(0, 0)
            .get_edge(RIGHT)
            .fits_with(p.get_piece(0, 1).get_edge(LEFT)),
        true
    );
    assert_eq!(
        p.get_piece(0, 0)
            .get_edge(BOTTOM)
            .fits_with(p.get_piece(0, 1).get_edge(LEFT)),
        false
    );
    assert_eq!(
        p.get_piece(0, 2)
            .get_edge(BOTTOM)
            .fits_with(p.get_piece(1, 2).get_edge(TOP)),
        true
    );
    assert_eq!(
        p.get_piece(0, 0)
            .get_edge(LEFT)
            .fits_with(p.get_piece(0, 0).get_edge(TOP)),
        false
    );
}
