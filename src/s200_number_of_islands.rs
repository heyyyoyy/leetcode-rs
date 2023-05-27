use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn bfs(row: usize, col: usize, grid: &mut Vec<Vec<char>>) {
        let (row_len, col_len) = (grid.len() as i32, grid[0].len() as i32);
        let delta = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut queue = VecDeque::with_capacity(grid.len());
        queue.push_back((row, col));
        grid[row][col] = '2';
        while let Some((cur_row, cur_col)) = queue.pop_front() {
            for (delta_row, delta_col) in delta {
                let (new_row, new_col) = (cur_row as i32 + delta_row, cur_col as i32 + delta_col);
                if new_row >= 0
                    && new_row < row_len
                    && new_col >= 0
                    && new_col < col_len
                    && grid[new_row as usize][new_col as usize] == '1'
                {
                    grid[new_row as usize][new_col as usize] = '2';
                    queue.push_back((new_row as usize, new_col as usize));
                }
            }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (row_len, col_len) = (grid.len(), grid[0].len());
        let mut island_count = 0;
        for row in 0..row_len {
            for col in 0..col_len {
                if grid[row][col] == '1' {
                    island_count += 1;
                    Self::bfs(row, col, &mut grid);
                }
            }
        }
        island_count
    }
}
