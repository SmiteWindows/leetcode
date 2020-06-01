// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// Runtime: 4 ms
// Memory Usage: 2.9 MB
use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let ic = s.char_indices().collect::<Vec<_>>();
    let len = ic.len();
    let (mut res, mut start) = (0, 0);
    let mut set = HashMap::with_capacity(len);
    for (idx, ch) in ic {
        if set.contains_key(&ch) {
            start = start.max(set[&ch]);
        }
        res = res.max(idx - start + 1);
        set.insert(ch, idx + 1);
    }
    res as i32
}
// hash_table two_pointers string sliding_window
#[test]
fn test4_3() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
}
