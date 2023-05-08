struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let (mut l, mut r) = (0, mat.len() - 1);
        mat.into_iter()
            .map(|v| {
                let mut cur = v[l];
                if l != r {
                    cur += v[r];
                }
                l += 1;
                r = r.checked_sub(1).unwrap_or_default();
                cur
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_sum() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::diagonal_sum(v), 25);
        let v = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(Solution::diagonal_sum(v), 8);
        let v = vec![vec![5]];
        assert_eq!(Solution::diagonal_sum(v), 5);
    }
}
