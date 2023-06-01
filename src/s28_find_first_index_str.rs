struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|idx| idx as i32).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(Solution::str_str("sadbutsad".to_owned(), "sad".to_owned()), 0);
        assert_eq!(Solution::str_str("leetcode".to_owned(), "leeto".to_owned()), -1);
    }
}