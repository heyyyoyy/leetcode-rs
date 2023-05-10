struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lenght_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_owned()),
            4
        );
    }
}
