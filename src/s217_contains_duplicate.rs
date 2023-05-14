use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate_iter(nums: Vec<i32>) -> bool {
        nums.len() != nums.iter().collect::<HashSet<_>>().len()
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hset = HashSet::with_capacity(nums.len());
        for n in nums {
            if hset.contains(&n) {
                return true;
            }
            hset.insert(n);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}