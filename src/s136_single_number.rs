use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(HashMap::new(), |mut hmap, num| {
                hmap.entry(num).and_modify(|n| *n += 1).or_insert(1);
                hmap
            })
            .into_iter()
            .find_map(|(key, val)| if val == 1 { Some(key) } else { None })
            .unwrap_or(-1)
    }

    pub fn single_number_bitwise(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .reduce(|prev, cur| prev ^ cur)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[test]
    fn test_single_number_bitwise() {
        assert_eq!(Solution::single_number_bitwise(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number_bitwise(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number_bitwise(vec![1]), 1);
    }
}
