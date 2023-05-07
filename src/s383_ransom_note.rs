use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut hmap = magazine.chars().fold(HashMap::new(), |mut acc, ch| {
            acc.entry(ch).and_modify(|count| *count += 1).or_insert(1);
            acc
        });
        for ch in ransom_note.chars() {
            if let Some(count) = hmap.get(&ch) {
                if *count > 0 {
                    hmap.entry(ch).and_modify(|count| *count -= 1);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "ab".to_owned()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "aab".to_owned()),
            true
        );
    }
}
