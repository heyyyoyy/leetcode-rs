struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut result = 0;
        let msb = (x as f64).log2() as i32;
        let mut temp = 1 << msb;
        while temp != 0 {
            let sum: i64 = result + temp;
            let y = sum.checked_mul(sum);
            if y <= Some(x as i64) {
                result += temp;
            }
            temp >>= 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
