// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hm = HashMap::new();
    let mut max = 0;
    let mut l = 0;
    for (r, c) in s.char_indices() {
        if let Some(end) = hm.insert(c, r) {
            l = l.max(end + 1);
        }
        max = max.max(r - l + 1);
    }
    max as i32
}
// hash_table two_pointers string sliding_window
#[test]
fn test2_3() {
    assert_eq!(length_of_longest_substring("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring("bbbbb")), 1);
    assert_eq!(length_of_longest_substring("pwwkew")), 3);
}
