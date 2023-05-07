use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut hmap = t.chars().fold(HashMap::new(), |mut acc, ch| {
            acc.entry(ch).and_modify(|i| *i += 1).or_insert(1);
            acc
        });
        for s_ch in s.chars() {
            let c = hmap.get(&s_ch);
            if let Some(count) = c {
                if *count == 0 {
                    return false;
                } else {
                    hmap.entry(s_ch).and_modify(|i| *i -= 1);
                }
            } else {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_anagram("car".to_owned(), "rat".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_anagram("aacc".to_owned(), "ccac".to_owned()),
            false
        );
    }
}
