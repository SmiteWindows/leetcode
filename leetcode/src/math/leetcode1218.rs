// https://leetcode-cn.com/problems/longest-arithmetic-subsequence-of-given-difference/
// Runtime: 28 ms
// Memory Usage: 3.5 MB
use std::collections::HashMap;
pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut res = 0;
    for x in arr {
        let prev = if let Some(&size) = hm.get(&(x - difference)) {
            size
        } else {
            0
        };
        let count = hm.entry(x).or_default();
        *count = (*count).max(prev + 1);
        res = res.max(*count);
    }
    res as i32
}
// math dynamic_programming
#[test]
fn test1_1218() {
    assert_eq!(longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    assert_eq!(longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    assert_eq!(longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
}
