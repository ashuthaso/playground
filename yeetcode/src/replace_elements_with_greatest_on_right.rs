//! https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/

//! Given an array arr, replace every element in that array with the greatest
//! element among the elements to its right, and replace the last element with -1.

#![allow(dead_code)]

struct ReplaceElementsWithGreatestOnTheRight;

impl ReplaceElementsWithGreatestOnTheRight {
    fn approach_1(arr: Vec<i32>) -> Vec<i32> {
        // Time complexity: O(n^2)
        // Space complexity: O(1)
        let mut arr = arr;

        let n = arr.len();
        for i in 0..(n - 1) {
            let mut right_max = arr[i + 1];
            for j in (i + 2)..n {
                right_max = std::cmp::max(arr[j], right_max);
            }
            arr[i] = right_max;
        }
        arr[n - 1] = -1;
        arr
    }
    fn approach_2(arr: Vec<i32>) -> Vec<i32> {
        // Time complexity: O(n)
        // Space complexity: O(1)
        let mut arr = arr;

        let mut right_max = -1;
        for i in (0..arr.len()).rev() {
            let new_max = std::cmp::max(arr[i], right_max);
            arr[i] = right_max;
            right_max = new_max;
        }
        arr
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(
        ReplaceElementsWithGreatestOnTheRight::approach_1(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
    assert_eq!(
        ReplaceElementsWithGreatestOnTheRight::approach_1(vec![400]),
        vec![-1]
    );
}

#[test]
fn test_approach_2() {
    assert_eq!(
        ReplaceElementsWithGreatestOnTheRight::approach_2(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
    assert_eq!(
        ReplaceElementsWithGreatestOnTheRight::approach_2(vec![400]),
        vec![-1]
    );
}
