struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 1;
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        nums.truncate(idx);
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut v = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v), 2);
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn test_remove_duplicates_2() {
        let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut v), 5);
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }
}
