fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num) in nums.iter().enumerate() {
        let x = target - num;
        if let Some(index) = nums.iter().position(|&n| x == n ) {
            if i == index {
                continue;
            }
            let mut new_vec = vec![i as i32, index as i32];
            new_vec.sort();
            return new_vec;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), [0, 1]);
        assert_eq!(two_sum(vec![3,2,4], 6), [1, 2]);
        assert_eq!(two_sum(vec![3,3], 6), [0, 1]);
    }
}
