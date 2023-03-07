use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap = HashMap::with_capacity(nums.len());
        for (idx, value) in nums.iter().enumerate() {
            let x = target - value;
            if let Some(&i) = hmap.get(&x) {
                return vec![i as i32, idx as i32];
            } else {
                hmap.insert(*value, idx);
            }
        }
        vec![]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), [0, 1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), [1, 2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), [0, 1]);
    }
}
