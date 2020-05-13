// https://leetcode.com/problems/contains-duplicate/
// Runtime: 4 ms
// Memory Usage: 2.7 MB
use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hs= HashSet::new();
    for n in nums {
        if !hs.insert(n) {
            return true;
        }
    }
    false
}
// hash_table array
#[test]
fn test1_217() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}
