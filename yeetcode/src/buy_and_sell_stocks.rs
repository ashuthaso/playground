//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

//! You are given an array prices where prices[i] is the price of a
//! given stock on the ith day.

//! You want to maximize your profit by choosing a single day to buy
//! one stock and choosing a different day in the future to sell that stock.

//! Return the maximum profit you can achieve from this transaction.
//! If you cannot achieve any profit, return 0.

#![allow(dead_code)]

struct BestTimeToBuyAndSellStock;

impl BestTimeToBuyAndSellStock {
    fn solution(prices: Vec<i32>) -> i32 {
        // Time complexity: O(n)
        // Space complexity: O(1)
        let mut buy = 0;
        let mut sell = 1;

        let mut max_profit = 0;

        while sell < prices.len() {
            if prices[buy] < prices[sell] {
                let profit = prices[sell] - prices[buy];
                if profit > max_profit {
                    max_profit = profit;
                }
            } else {
                buy = sell;
            }
            sell += 1;
        }
        max_profit
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        BestTimeToBuyAndSellStock::solution(vec![7, 1, 5, 3, 6, 4]),
        5
    );
    assert_eq!(BestTimeToBuyAndSellStock::solution(vec![7, 6, 4, 3, 1]), 0);
}
