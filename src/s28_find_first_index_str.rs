struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|idx| idx as i32).unwrap_or(-1)
    }

    pub fn str_str_window(haystack: String, needle: String) -> i32 {
        let (n, m) = (haystack.len(), needle.len());
        if n < m {
            return -1;
        }
        for window_idx in 0..n - m + 1 {
            for i in 0..m {
                if needle.chars().nth(i).unwrap() != haystack.chars().nth(window_idx + i).unwrap() {
                    break;
                }
                if i == m - 1 {
                    return window_idx as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_owned(), "sad".to_owned()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_owned(), "leeto".to_owned()),
            -1
        );
    }

    #[test]
    fn test_str_str_window() {
        assert_eq!(
            Solution::str_str_window("sadbutsad".to_owned(), "sad".to_owned()),
            0
        );
        assert_eq!(
            Solution::str_str_window("leetcode".to_owned(), "leeto".to_owned()),
            -1
        );
        assert_eq!(
            Solution::str_str_window("abb".to_owned(), "abaaa".to_owned()),
            -1
        );
    }
}
