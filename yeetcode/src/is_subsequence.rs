//! https://leetcode.com/problems/is-subsequence/

//! Given two strings s and t, return true if s is a subsequence of t,
//! or false otherwise.

#![allow(dead_code)]

struct IsSubsequence;

impl IsSubsequence {
    fn solution(s: String, t: String) -> bool {
        // Time complexity: O(n) ; n = length(t)
        // Space complexity: O(s + t)
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();

        let mut s_idx = 0;
        let mut t_idx = 0;

        while s_idx < s.len() {
            if t_idx == t.len() {
                return false;
            }
            if s[s_idx] == t[t_idx] {
                s_idx += 1;
            }
            t_idx += 1;
        }
        true
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        IsSubsequence::solution(String::from("abc"), String::from("ahbgdc")),
        true
    );
    assert_eq!(
        IsSubsequence::solution(String::from("axc"), String::from("ahbgdc")),
        false
    );
}
