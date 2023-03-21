use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_palindrome_2(s: String) -> i32 {
        let mut lenght = 0;
        let hmap = s.chars().fold(HashMap::new(), |mut acc, ch| {
            acc.entry(ch).and_modify(|count| *count += 1).or_insert(1);
            acc
        });
        for (_, count) in hmap.iter() {
            lenght += count / 2 * 2;
            if lenght % 2 == 0 && count % 2 == 1 {
                lenght += 1;
            }
        }
        lenght
    }

    pub fn longest_palindrome(s: String) -> i32 {
        (s.chars()
            .fold(HashMap::new(), |mut acc, ch| {
                acc.entry(ch).and_modify(|count| *count += 1).or_insert(1);
                acc
            })
            .iter()
            .map(|(_, &count)| count & !1)
            .sum::<i32>()
            + 1)
        .min(s.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_owned()), 7);
        assert_eq!(Solution::longest_palindrome("a".to_owned()), 1);
    }
}
