//! https://leetcode.com/problems/valid-palindrome/

//! Given a string s, return true if it is a palindrome, or false otherwise.

#![allow(dead_code)]

struct ValidPalindrome;

impl ValidPalindrome {
    fn approach_1(s: String) -> bool {
        // Time complexity: O(n)
        // Space complexity: O(n)
        let string = s.to_lowercase();
        let filtered_chars = string.chars().filter(|char| char.is_alphanumeric());

        let original = filtered_chars.clone().collect::<String>();
        let reversed = filtered_chars.rev().collect::<String>();

        original == reversed
    }

    fn approach_2(s: String) -> bool {
        // Time complexity: O(n)
        // Space complexity: O(1)
        let chars = s.chars().collect::<Vec<_>>();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            while left < right && !chars[left].is_alphanumeric() {
                left += 1;
            }
            while left < right && !chars[right].is_alphanumeric() {
                right = right.saturating_sub(1);
            }

            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            right = right.saturating_sub(1);
        }
        true
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(
        ValidPalindrome::approach_1(String::from("A man, a plan, a canal: Panama")),
        true
    );
    assert_eq!(
        ValidPalindrome::approach_1(String::from("race a car")),
        false
    );
    assert_eq!(ValidPalindrome::approach_1(String::from(" ")), true);
    assert_eq!(ValidPalindrome::approach_1(String::from("a.")), true);
}

#[test]
fn test_approach_2() {
    assert_eq!(
        ValidPalindrome::approach_2(String::from("A man, a plan, a canal: Panama")),
        true
    );
    assert_eq!(
        ValidPalindrome::approach_2(String::from("race a car")),
        false
    );
    assert_eq!(ValidPalindrome::approach_2(String::from(" ")), true);
    assert_eq!(ValidPalindrome::approach_2(String::from("a.")), true);
}
