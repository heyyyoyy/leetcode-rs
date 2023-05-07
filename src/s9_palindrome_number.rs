struct Solution;

impl Solution {
    fn digit_count(mut x: i32) -> i32 {
        let mut count = 0;
        while x != 0 {
            x /= 10;
            count += 1;
        }
        count
    }

    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let count = Self::digit_count(x);
        if count == 1 {
            return true
        }
        let start = x;
        let mut new = 0;
        for i in (0..count).rev() {
            let rem = x % 10;
            new += rem * 10_i32.pow(i as u32);
            x /= 10;
        }
        start == new
    }

    pub fn is_palindrome_string(x: i32) -> bool {
        let x_string = x.to_string();
        x_string == x_string.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test_is_palindrome_string() {
        assert_eq!(Solution::is_palindrome_string(121), true);
        assert_eq!(Solution::is_palindrome_string(-121), false);
        assert_eq!(Solution::is_palindrome_string(10), false);
    }
}