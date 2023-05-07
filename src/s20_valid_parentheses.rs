use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let hmap = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        let mut stack = vec![];
        for char in s.chars() {
            let open_bracket = hmap.get(&char);
            if open_bracket.is_none() {
                stack.push(char);
            } else {
                let stack_open = stack.pop();
                if stack_open.is_none() {
                    return false;
                }
                if *open_bracket.unwrap() != stack_open.unwrap() {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_owned()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_owned()), true);
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
        assert_eq!(Solution::is_valid("(".to_owned()), false);
        assert_eq!(Solution::is_valid("]".to_owned()), false);
    }
}
