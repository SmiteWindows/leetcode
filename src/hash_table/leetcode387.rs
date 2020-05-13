// https://leetcode.com/problems/first-unique-character-in-a-string/
// Runtime: 16 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn first_uniq_char(s: String) -> i32 {
    let mut hm= HashMap::new();
    for c in s.chars() {
        *hm.entry(c).or_default() += 1;
    }
    for (i, c) in s.chars().enumerate() {
        if let Some(1) = hm.get(&c) {
            return i as i32;
        }
    }
    -1
}
// hash_table string
#[test]
fn test1_387() {
    assert_eq!(first_uniq_char(String::from("leetcode")), 0);
    assert_eq!(first_uniq_char(String::from("loveleetcode")), 2);
}
