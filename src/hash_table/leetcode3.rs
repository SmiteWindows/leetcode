// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// Runtime: 252 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let n = s.len();
    let mut res = 0;
    let mut map = HashMap::new();
    let mut i = 0;
    for j in 0..n {
        let ch = s.chars().nth(j).expect("exist");
        if map.contains_key(&ch) {
            i = i.max(map[&ch]);
        }
        res = res.max(j - i + 1);
        map.insert(ch, j + 1);
    }
    res as i32
}
// hash_table two_pointers string sliding_window
#[test]
fn test3_3() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
}
