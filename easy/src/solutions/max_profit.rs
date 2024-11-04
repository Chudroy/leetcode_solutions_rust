use crate::models::Solution;

/**
 * 121. Best Time to Buy and Sell Stock
 * You are given an array prices where
 * prices[i] is the price of a given
 * stock on the ith day.
 * You want to maximize your profit by
 * choosing a single day to buy one stock
 * and choosing a different day in the future to sell that stock.
 * Return the maximum profit you can achieve from this transaction. 
 * If you cannot achieve any profit, return 0.
 */
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            if price < min_price {
                min_price = price;
            } else if price - min_price > max_profit {
                max_profit = price - min_price;
            }
        }

        max_profit
    }
}
