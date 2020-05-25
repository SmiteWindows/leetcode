// https://leetcode.com/problems/valid-anagram/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for sc in s.chars() {
        *hm.entry(sc).or_default() += 1;
    }
    for tc in t.chars() {
        *hm.entry(tc).or_default() -= 1;
    }
    hm.values().all(|&x| x == 0)
}
// sort hash_table
#[test]
fn test2_242() {
    assert_eq!(
        is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
}
