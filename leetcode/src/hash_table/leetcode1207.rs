// // https://leetcode-cn.com/problems/unique-number-of-occurrences/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::{HashMap, HashSet};
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for x in arr {
        *hm.entry(x).or_default() += 1;
    }
    let mut hs = HashSet::new();
    for x in hm.values() {
        if !hs.insert(x) {
            return false;
        }
    }
    true
}
// hash_table
#[test]
fn test1_1207() {
    assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(unique_occurrences(vec![1, 2]), false);
    assert_eq!(
        unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
        true
    );
}
