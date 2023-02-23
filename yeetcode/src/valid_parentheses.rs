//! https://leetcode.com/problems/valid-parentheses/

//! Given a string s containing just the characters '(', ')', '{', '}',
//! '[' and ']', determine if the input string is valid.

//! An input string is valid if:
//!    Open brackets must be closed by the same type of brackets.
//!    Open brackets must be closed in the correct order.
//!    Every close bracket has a corresponding open bracket of the same type.

#![allow(dead_code)]

struct ValidParentheses;

impl ValidParentheses {
    fn approach_1(s: String) -> bool {
        // Time complexity: O(n)
        // Space complexity: O(n)
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            match char {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                _ => {
                    if stack.pop() != Some(char) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(ValidParentheses::approach_1(String::from("()")), true);
    assert_eq!(ValidParentheses::approach_1(String::from("()[]{}")), true);
    assert_eq!(ValidParentheses::approach_1(String::from("(]")), false);
}
