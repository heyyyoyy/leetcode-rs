use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hmap = HashMap::new();
        let mut substring_len = 0;
        let mut start = -1;
        for (idx, ch) in s.chars().enumerate() {
            if let Some(old_idx) = hmap.insert(ch, idx as i32) {
                start = start.max(old_idx);
            }
            substring_len = substring_len.max(idx as i32 - start);
        }
        substring_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
    }
}
