use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row = grid.len();
        let col = grid[0].len();
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; col]; row];
        for (r, v) in grid.iter().enumerate() {
            for (c, _) in v.iter().enumerate() {
                let mut left = c as i32 - 1;
                let mut top = r as i32 - 1;
                let mut st = HashSet::new();
                while left >= 0 && top >= 0 {
                    st.insert(grid[top as usize][left as usize]);
                    left -= 1;
                    top -= 1;
                }
                matrix[r][c] = st.len() as i32;
                st.clear();

                let mut bottom = r + 1;
                let mut right = c + 1;
                while bottom < row && right < col {
                    st.insert(grid[bottom as usize][right as usize]);
                    bottom += 1;
                    right += 1;
                }
                if matrix[r][c] > st.len() as i32 {
                    matrix[r][c] -= st.len() as i32
                } else {
                    matrix[r][c] = (st.len() as i32) - matrix[r][c]
                }
            }
        }
        matrix
    }
}
