// https://leetcode.com/problems/two-sum/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_map: HashMap<i32, usize> = nums
        .iter()
        .enumerate()
        .map(|(index, value)| (target - value, index))
        .collect();
    for (index, value) in nums.iter().enumerate() {
        if let Some(&j) = nums_map.get(value) {
            if index != j {
                return vec![index as i32, j as i32];
            }
        }
    }
    unreachable!();
}
// array hash_table
#[test]
fn test1_1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
