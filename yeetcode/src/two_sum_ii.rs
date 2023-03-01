//! https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

//! Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number.

#![allow(dead_code)]

use std::cmp::Ordering;

struct TwoSumII;

impl TwoSumII {
    fn solution(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // Time complexity: O(n)
        // Space complexity: O(1)
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
            }
        }
        unreachable!();
    }
}

#[test]
fn test_solution() {
    assert_eq!(TwoSumII::solution(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(TwoSumII::solution(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(TwoSumII::solution(vec![-1, 0], -1), vec![1, 2]);
}
