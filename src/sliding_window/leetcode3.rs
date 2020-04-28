// https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::{cmp::max, collections::HashMap};
// Runtime: 4 ms
// Memory Usage: 2.9 MB
pub fn length_of_longest_substring(s: String) -> i32 {
    let ic: Vec<(usize, char)> = s.char_indices().collect();
    let len = ic.len();
    let (mut ans, mut start) = (0, 0);
    let mut set = HashMap::with_capacity(len);
    for (idx, ch) in ic {
        if set.contains_key(&ch) {
            start = max(set[&ch], start);
        }
        ans = max(ans, idx - start + 1);
        set.insert(ch, idx + 1);
    }
    ans as i32
}
// hash_table two_pointers string sliding_window
#[test]
fn test4_3() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
}
