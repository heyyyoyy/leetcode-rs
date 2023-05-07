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

    pub fn longest_common_prefix_reduce(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, cur| {
                acc.chars()
                    .zip(cur.chars())
                    .take_while(|(f, s)| f == s)
                    .map(|(f, _)| f)
                    .collect()
            })
            .unwrap()
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

    #[test]
    fn test_longest_common_prefix_reduce() {
        assert_eq!(
            Solution::longest_common_prefix_reduce(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix_reduce(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            ""
        );
    }
}
