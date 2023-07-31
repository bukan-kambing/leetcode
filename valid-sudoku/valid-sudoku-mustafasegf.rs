use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0; 9];
        let mut cols = [0; 9];
        let mut grid = [0; 9];

        for row in 0..9 {
            for col in 0..9 {
                if let Some(num) = board[row][col].to_digit(10) {
                    let grid_idx = 3 * (row / 3) + (col / 3);
                    let num = 1 << num;

                    if (rows[row] & num) != 0
                        || (cols[col] & num) != 0
                        || (grid[grid_idx] & num) != 0
                    {
                        return false;
                    }

                    rows[row] |= num;
                    cols[col] |= num;
                    grid[grid_idx] |= num;
                }
            }
        }

        return true;
    }
}
