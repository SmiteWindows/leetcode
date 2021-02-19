// https://leetcode-cn.com/problems/search-insert-position/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::convert::identity;
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(identity) as i32
}
// binary_search array
#[test]
fn test2_35() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
}
