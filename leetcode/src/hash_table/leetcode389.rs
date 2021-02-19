// https://leetcode-cn.com/problems/find-the-difference/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn find_the_difference(s: String, t: String) -> char {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *map.entry(c).or_default() += 1;
    }
    for c in s.chars() {
        *map.entry(c).or_default() -= 1;
    }
    for (&c, &v) in map.iter() {
        if v == 1 {
            return c;
        }
    }
    unreachable!()
}
// bit_manipulation hash_table
#[test]
fn test2_389() {
    assert_eq!(
        find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
}
