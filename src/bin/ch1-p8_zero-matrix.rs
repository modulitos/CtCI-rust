// Zero Matrix: Write an algorithm such that if an element in an MxN
// matrix is 0, its entire row and column are set to O.

use std::collections::HashSet;

type Matrix<'a> = &'a mut [&'a mut [u32]];

fn zero_matrix(matrix: Matrix) -> Matrix {
    let mut zero_rows = HashSet::new();
    let mut zero_cols = HashSet::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                zero_rows.insert(i);
                zero_cols.insert(j);
            }
        }
    }
    // now update our matrix with zero's:
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if zero_rows.contains(&i) || zero_cols.contains(&j) {
                matrix[i][j] = 0;
                // continue;
            }
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
            zero_matrix(&mut [&mut [1, 2], &mut [3, 4]]),
            [[1, 2], [3, 4]]
        );
        assert_eq!(
            zero_matrix(&mut [&mut [0, 2], &mut [3, 4]]),
            [[0, 0], [0, 4]]
        );
        assert_eq!(
            zero_matrix(&mut [
                &mut [1, 2, 3, 4],
                &mut [5, 0, 7, 8],
                &mut [9, 10, 11, 12],
                &mut [13, 14, 15, 16]
            ]),
            [[1, 0, 3, 4], [0, 0, 0, 0], [9, 0, 11, 12], [13, 0, 15, 16]]
        );
    }
}

fn main() {}
