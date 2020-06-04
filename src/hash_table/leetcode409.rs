// https://leetcode.com/problems/longest-palindrome/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn longest_palindrome(s: String) -> i32 {
    let mut hs = HashSet::new();
    let mut half = 0;
    for c in s.chars() {
        if hs.contains(&c) {
            hs.remove(&c);
            half += 1;
        } else {
            hs.insert(c);
        }
    }
    if hs.is_empty() {
        2 * half
    } else {
        2 * half + 1
    }
}
// hash_table
#[test]
fn test1_409() {
    assert_eq!(longest_palindrome(String::from("abccccdd")), 7);
}
