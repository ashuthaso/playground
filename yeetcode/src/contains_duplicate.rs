//! https://leetcode.com/problems/contains-duplicate/

//! Given an integer array nums, return true if any value appears at least
//! twice in the array, and return false if every element is distinct.

#![allow(dead_code)]

use std::collections::HashSet;

struct ContainsDuplicate;

impl ContainsDuplicate {
    fn approach_1(nums: Vec<i32>) -> bool {
        // Time complexity: O(n^2)
        // Space complexity: O(1)
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }

    fn approach_2(nums: Vec<i32>) -> bool {
        // Time complexity: O(n)
        // Space complexity: O(n)
        let mut lookup = HashSet::new();
        for elem in &nums {
            if lookup.contains(elem) {
                return true;
            }
            lookup.insert(elem);
        }
        false
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(ContainsDuplicate::approach_1(vec![1, 2, 3, 1]), true);
    assert_eq!(ContainsDuplicate::approach_1(vec![1, 2, 3, 4]), false);
    assert_eq!(
        ContainsDuplicate::approach_1(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}

#[test]
fn test_approach_2() {
    assert_eq!(ContainsDuplicate::approach_2(vec![1, 2, 3, 1]), true);
    assert_eq!(ContainsDuplicate::approach_2(vec![1, 2, 3, 4]), false);
    assert_eq!(
        ContainsDuplicate::approach_2(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}
