//! https://leetcode.com/problems/binary-search/

//! Given an array of integers nums which is sorted in ascending order,
//! and an integer target, write a function to search target in nums.
//! If target exists, then return its index. Otherwise, return -1.

//! You must write an algorithm with O(log n) runtime complexity.

struct BinarySearch;

impl BinarySearch {
    fn solution(nums: Vec<i32>, target: i32) -> i32 {
        // Time complexity: O(log n)
        // Space complexity: O(1)
        let (mut low, mut high) = (0, (nums.len() - 1) as i32);

        while low <= high {
            let mid = (low + high) / 2;

            if nums[mid as usize] < target {
                low = mid + 1;
            } else if nums[mid as usize] > target {
                high = mid - 1;
            } else {
                return mid;
            }
        }
        -1
    }
}

#[test]
fn test_solution() {
    assert_eq!(BinarySearch::solution(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(BinarySearch::solution(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(BinarySearch::solution(vec![5], -5), -1);
}
