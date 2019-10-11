// Jigsaw: Implement an NxN jigsaw puzzle. Design the data structures
// and explain an algorithm to solve the puzzle. You can assume that
// you have a fits_with method which, when passed two puzzle edges,
// returns true if the two edges belong together.

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Orientation {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

#[derive(PartialEq, Eq)]
enum Shape {
    IN,
    OUT,
    FLAT,
}

type EdgeId = (u8, u8);

struct Edge {
    id: EdgeId,
    shape: Shape,
}

impl Edge {
    fn new(row: u8, col: u8, shape: Shape) -> Self {
        Edge {
            id: (row, col),
            shape,
        }
    }
}
struct Piece {
    edges: HashMap<Orientation, Edge>,
}

impl Piece {
    fn new(row: u8, col: u8, rows: u8, cols: u8) -> Self {
        let mut edges = HashMap::new();
        edges.insert(
            Orientation::LEFT,
            Edge::new(row, col, if col == 0 { Shape::FLAT } else { Shape::OUT }),
        );
        edges.insert(
            Orientation::TOP,
            Edge::new(row, col, if row == 0 { Shape::FLAT } else { Shape::OUT }),
        );
        edges.insert(
            Orientation::RIGHT,
            Edge::new(
                row,
                col + 1,
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
                row + 1,
                col,
                if row == rows - 1 {
                    Shape::FLAT
                } else {
                    Shape::IN
                },
            ),
        );
        Piece { edges }
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

    fn shuffle(&mut self) {}

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
