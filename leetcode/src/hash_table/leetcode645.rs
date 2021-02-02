// https://leetcode-cn.com/problems/set-mismatch/
// Runtime: 12 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut hs = HashSet::new();
    let mut res = Vec::new();
    for x in nums {
        if !hs.insert(x) {
            res.push(x);
        }
    }
    for i in 1..=n {
        if hs.insert(i as i32) {
            res.push(i as i32);
        }
    }
    res
}
// math hash_table
#[test]
fn test2_645() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
}
