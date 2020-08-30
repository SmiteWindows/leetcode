// https://leetcode-cn.com/problems/two-sum/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_map = nums
        .iter()
        .enumerate()
        .map(|(index, &value)| (target - value, index))
        .collect::<HashMap<_, _>>();
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
fn test2_1() {
    assert_eq!(two_sum(vec![2, 11, 7, 9, 15], 9), vec![0, 2]);
}
