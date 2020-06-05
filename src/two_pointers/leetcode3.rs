// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// Runtime: 732 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {
    // TODO
    let n = s.len();
    let mut set = HashSet::new();
    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    while i < n && j < n {
        let ch = s.chars().nth(j).expect("exist");
        if set.contains(&ch) {
            let ch = s.chars().nth(i).expect("exist");
            set.remove(&ch);
            i += 1;
        } else {
            set.insert(ch);
            j += 1;
            res = res.max(j - i);
        }
    }
    res as i32
}
// hash_table two_pointers string sliding_window
#[test]
fn test2_3() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
}
