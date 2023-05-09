struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|el| *el != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut v = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut v, 3), 2);
        assert_eq!(v, vec![2, 2]);
    }

    #[test]
    fn test_remove_element_2() {
        let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut v, 2), 5);
        assert_eq!(v, vec![0, 1, 3, 0, 4]);
    }
}
