struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let left = s
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .map(|ch| ch.to_ascii_lowercase());
        let right = left.clone().rev();
        left.zip(right).all(|(x, y)| x == y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
        assert_eq!(Solution::is_palindrome(" ".to_owned()), true);
        assert_eq!(Solution::is_palindrome("0P".to_owned()), false)
    }
}
