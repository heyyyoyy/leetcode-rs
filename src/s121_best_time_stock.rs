struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_buy = prices[0];
        let mut max_profit = 0;
        for price in prices {
            let current_profit = price - min_buy;
            if current_profit > max_profit {
                max_profit = current_profit;
            }
            if min_buy > price {
                min_buy = price;
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
