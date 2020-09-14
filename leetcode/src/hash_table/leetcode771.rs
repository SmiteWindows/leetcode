// https://leetcode-cn.com/problems/jewels-and-stones/
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
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}
