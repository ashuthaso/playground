//! https://leetcode.com/problems/length-of-last-word/

//! Given a string s consisting of words and spaces, return the length
//! of the last word in the string.

#![allow(dead_code)]

struct LengthOfLastWord;

impl LengthOfLastWord {
    fn approach_1(s: String) -> i32 {
        // Time complexity: O(n)
        // Space complexity: O(n)

        let s = s.chars().collect::<Vec<_>>();
        let mut idx = s.len() - 1;
        let mut length = 0;

        // Ignore padding white spaces
        while s[idx].is_whitespace() {
            idx -= 1;
        }

        while !s[idx].is_whitespace() {
            length += 1;
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
        length
    }

    fn approach_2(s: String) -> i32 {
        let last_word = s.split_whitespace().rev().next().unwrap();
        last_word.len() as i32
    }
}

#[test]
fn test_approach_1() {
    assert_eq!(LengthOfLastWord::approach_1(String::from("Hello World")), 5);
    assert_eq!(
        LengthOfLastWord::approach_1(String::from("   fly me   to   the moon  ")),
        4
    );
    assert_eq!(
        LengthOfLastWord::approach_1(String::from("luffy is still joyboy")),
        6
    );
}

#[test]
fn test_approach_2() {
    assert_eq!(LengthOfLastWord::approach_2(String::from("Hello World")), 5);
    assert_eq!(
        LengthOfLastWord::approach_2(String::from("   fly me   to   the moon  ")),
        4
    );
    assert_eq!(
        LengthOfLastWord::approach_2(String::from("luffy is still joyboy")),
        6
    );
}
