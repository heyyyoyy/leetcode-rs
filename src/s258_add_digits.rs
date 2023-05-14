struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        match num {
            0 => 0,
            x if x % 9 == 0 => 9,
            x => x % 9
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        assert_eq!(Solution::add_digits(38), 2);
        assert_eq!(Solution::add_digits(0), 0);
    }
}
