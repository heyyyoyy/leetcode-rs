struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut s = String::new();
        let mut idx = 0;
        loop {
            let x: String = strs.iter().filter_map(|s| s.chars().nth(idx)).collect();
            if x.len() == strs.len() && x.chars().all(|ch| ch == x.chars().next().unwrap()) {
                s.push(x.chars().next().unwrap());
            } else {
                break;
            }
            idx += 1;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            ""
        );
    }
}
