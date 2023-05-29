struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let (mut sum, mut prev) = (0, 0);
        for i in s.chars().rev() {
            let cur = match i {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            if cur >= prev {
                sum += cur
            } else {
                sum -= cur
            }
            prev = cur
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
