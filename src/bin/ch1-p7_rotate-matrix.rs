// Rotate Matrix: Given an image represented by an NxN matrix, where
// each pixel in the image is 4 bytes, write a method to rotate the
// image by 90 degrees. (an you do this in place?

// 4 bytes means that i32 is adequate?
// i32 or u32 to represent each pixel?
// would a nested array be good enough, instead of a proper matrix struct?

// N=3
// (0,0) -> (0, 2))
// (0,2) -> (2, 2))
// (2,2) -> (2, 0))
// (2,0) -> (0, 0))

// (0,1) -> (1, 2)
// (1,2) -> (2, 1)
// (2,1) -> (1,0)
// (1,0) -> (0,1)

// (1,1) -> (1, 1)
fn get_90_degree(row: usize, col: usize, n: usize) -> (usize, usize) {
    let new_row = col;
    let new_col = n - row - 1;
    (new_row, new_col)
}

// NOTE: this trait isn't really needed, but it's a simple example of
// a trait for learning purposes
trait DivRoundUp {
    fn div_round_up(self, d: Self) -> Self;
    fn div_round_down(self, d: Self) -> Self;
}

impl DivRoundUp for usize {
    fn div_round_up(self, d: Self) -> Self {
        self / d + (self % d)
    }
    fn div_round_down(self, d: Self) -> Self {
        self / d
    }
}

type Image<'a> = &'a mut [&'a mut [u32]];

fn rotate_matrix(matrix: Image) -> Image {
    if matrix.len() != matrix[0].len() {
        panic!("matrix is not NxN!");
    }

    // loop over everything in the first quadrant, calling
    // "get_90_degree" to set it 4 times
    for row in 0..matrix.len().div_round_up(2) {
        // println!("iterating! row: {}", row);
        for col in 0..matrix[0].len().div_round_down(2) {
            let mut old_row = row;
            let mut old_col = col;
            let mut prev_pixel = matrix[row][col];
            println!("iterating! row: {}, col: {}", row, col);
            for i in 0..4 {
                let (new_row, new_col) = get_90_degree(old_row, old_col, matrix.len());
                println!(
                    "new_row: {}, new_col: {}, prev_pixel: {}, i: {}",
                    new_row, new_col, prev_pixel, i
                );
                let next_pixel = matrix[new_row][new_col];
                matrix[new_row][new_col] = prev_pixel;
                prev_pixel = next_pixel;
                old_row = new_row;
                old_col = new_col;
            }
        }
    }
    matrix
}

// CTCI solution:
#[allow(dead_code)]
fn rotate_matrix_CTCI(matrix: Image) -> Image {
    if matrix.len() != matrix[0].len() {
        panic!("matrix is not NxN!");
    }

    // work our way from outside the matrix to the inside of the matrix
    let n = matrix.len();
    for layer in 0..matrix.len().div_round_down(2) {
        // iterate over each layer, decreasing by 1 on the left side
        // to avoid rotating more than once
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            println!("iterating! first: {}, i: {}", first, i);

            let top = matrix[first][i]; // save top
                                        // left -> top
            matrix[first][i] = matrix[last - offset][first];
            // bottom -> left
            matrix[last - offset][first] = matrix[last][last - offset];
            // right -> bottom
            matrix[last][last - offset] = matrix[i][last];
            // top -> right
            matrix[i][last] = top;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            rotate_matrix(&mut [&mut [1, 2], &mut [3, 4]]),
            [[3, 1], [4, 2]]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            rotate_matrix(&mut [&mut [1, 2, 3], &mut [4, 5, 6], &mut [7, 8, 9]]),
            [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            rotate_matrix(&mut [
                &mut [1, 2, 3, 4],
                &mut [5, 6, 7, 8],
                &mut [9, 10, 11, 12],
                &mut [13, 14, 15, 16]
            ]),
            [
                [13, 9, 5, 1],
                [14, 10, 6, 2],
                [15, 11, 7, 3],
                [16, 12, 8, 4]
            ]
        );
    }
}

fn main() {}
