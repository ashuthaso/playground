//! https://leetcode.com/problems/two-sum/

//! Given an array of integers nums and an integer target, return
//! indices of the two numbers such that they add up to target.

#![allow(dead_code)]

use std::collections::HashMap;

struct TwoSum;

impl TwoSum {
    fn approach_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Time complexity: O(n^2)
        // Space complexity: O(1)
        for (i, &a) in nums.iter().enumerate() {
            for (j, &b) in nums.iter().enumerate() {
                if i != j && a + b == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
    fn approach_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Time complexity: O(n)
        // Space complexity: O(n)
        let mut lookup = HashMap::new();
        for (idx, &val) in nums.iter().enumerate() {
            let diff = target - val;
            if let Some(&prev_idx) = lookup.get(&diff) {
                return vec![prev_idx, idx as i32];
            }
            lookup.insert(val, idx as i32);
        }
        vec![]
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(TwoSum::approach_1(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(TwoSum::approach_1(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(TwoSum::approach_1(vec![3, 3], 6), vec![0, 1]);
}

#[test]
fn test_approach_2() {
    assert_eq!(TwoSum::approach_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(TwoSum::approach_2(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(TwoSum::approach_2(vec![3, 3], 6), vec![0, 1]);
}
