// https://leetcode.com/problems/two-sum/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();

    for (i, &e) in nums.iter().enumerate() {
        if let Some(&v) = hm.get(&(target - e)) {
            return vec![v, i as i32];
        }
        hm.insert(e, i as i32);
    }
    vec![]
}
// array hash_table
#[test]
fn test1_1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
