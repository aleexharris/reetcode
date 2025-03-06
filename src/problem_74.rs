use std::cmp::Ordering;

impl Solution {
    // Remember: len * len -1 for indices, not just len * len
    // Modulo % for the remainder, instead of manual calculation
    // Do math with i32 and then convert to usize JIT for indexing
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let x_len = matrix.len() as i32;
        let y_len = matrix[0].len() as i32;
        let (mut left, mut right) = (0, x_len * y_len - 1);
        loop {
            if left > right {
                break false
            }
            let mut mid = left + (right - left) / 2;
            let mut x = mid / y_len;
            let mut y = mid % y_len;
            match matrix[x as usize][y as usize].cmp(&target) {
                Ordering::Equal => break true,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid - 1,
            }
        }
    }
}