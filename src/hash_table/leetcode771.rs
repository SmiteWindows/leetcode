// https://leetcode.com/problems/jewels-and-stones/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let mut hs = HashSet::new();
    for c in j.chars() {
        hs.insert(c);
    }
    let mut res = 0;
    for c in s.chars() {
        if hs.contains(&c) {
            res += 1;
        }
    }
    res
}
// hash_table
#[test]
fn test1_771() {
    assert_eq!(
        num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")),
        3
    );
    assert_eq!(
        num_jewels_in_stones(String::from("z"), String::from("ZZ")),
        0
    );
}
