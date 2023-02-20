//! https://leetcode.com/problems/valid-anagram/

//! Given two strings s and t, return true if t is an anagram of s, and false otherwise.

#![allow(dead_code)]

use std::collections::HashMap;

struct ValidAnagram;

impl ValidAnagram {
    fn approach_1(s: String, t: String) -> bool {
        // Time complexity: O(n)
        // Space complexity: O(s + t)
        if s.len() != t.len() {
            return false;
        }

        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();

        for char in s.chars() {
            s_map
                .entry(char)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for char in t.chars() {
            t_map
                .entry(char)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for key in s_map.keys() {
            if s_map.get(key) != t_map.get(key) {
                return false;
            }
        }
        true
    }
    fn approach_2(s: String, t: String) -> bool {
        // Time complexity: O(n log n) for sorting
        // Space complexity: O(n/2) for large strings, O(1) for short strings

        if s.len() != t.len() {
            return false;
        }
        let mut s_chars = s.chars().collect::<Vec<_>>();
        let mut t_chars = t.chars().collect::<Vec<_>>();

        s_chars.sort();
        t_chars.sort();

        s_chars == t_chars
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(
        ValidAnagram::approach_1(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(
        ValidAnagram::approach_1(String::from("rat"), String::from("car")),
        false
    );
}

#[test]
fn test_approach_2() {
    assert_eq!(
        ValidAnagram::approach_2(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(
        ValidAnagram::approach_2(String::from("rat"), String::from("car")),
        false
    );
}
