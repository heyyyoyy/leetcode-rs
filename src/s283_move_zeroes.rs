struct Solution;

impl Solution {
    pub fn move_zeroes_space_suboptimal(nums: &mut Vec<i32>) {
        let zero_count = nums.iter().filter(|&&n| n == 0).count();

        let mut temp_arr = Vec::with_capacity(nums.len() - zero_count);
        for i in nums.iter() {
            if *i != 0 {
                temp_arr.push(*i);
            }
        }
        for _ in 0..zero_count {
            temp_arr.push(0);
        }
        for i in 0..nums.len() {
            nums[i] = temp_arr[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes_space_suboptimal() {
        let mut arr = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes_space_suboptimal(&mut arr);
        assert_eq!(vec![1, 3, 12, 0, 0], arr);
    }
}
