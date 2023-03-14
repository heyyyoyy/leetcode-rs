use std::collections::{HashSet, VecDeque};

struct Solution;


impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let delta = [(-1,0), (1,0), (0,-1), (0,1)];
        let row_len = image.len() as i32;
        let col_len = image[0].len() as i32;
        let pixel_color = image[sr as usize][sc as usize];
        let mut visited = HashSet::new();
        image[sr as usize][sc as usize] = color;
        let mut queue = VecDeque::new();
        queue.push_back((sr, sc));
        while let Some((row, col)) = queue.pop_front() {
            image[row as usize][col as usize] = color;
            visited.insert((row, col));
            for (row_delta, col_delta) in delta {
                let new_row = row + row_delta;
                let new_col = col + col_delta;
                if new_col < 0 || new_col >= col_len || 
                   new_row < 0 || new_row >= row_len ||
                   visited.contains(&(new_row, new_col)) {
                    continue;
                }
                if image[new_row as usize][new_col as usize] == pixel_color {
                    queue.push_back((new_row, new_col));
                }
            }
        }
        image
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_flood_fill() {

        assert_eq!(
            Solution::flood_fill(
                vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]], 1, 1, 2
            ), [[2,2,2],[2,2,0],[2,0,1]]
        );
        assert_eq!(
            Solution::flood_fill(
                vec![vec![0,0,0], vec![0,0,0]], 0, 0, 0
            ), [[0,0,0],[0,0,0]]
        );
        assert_eq!(
            Solution::flood_fill(
                vec![vec![0,0,0], vec![0,1,0]], 1, 1, 2
            ), [[0,0,0],[0,2,0]]
        );
    }
}