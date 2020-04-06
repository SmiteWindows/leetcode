// https://leetcode.com/problems/two-sum/
/// Runtime: 0 ms
/// Memory Usage: 2.2 MB
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let nums_map = nums
        .iter()
        .enumerate()
        .map(|(index, value)| (target - value, index))
        .collect::<HashMap<i32, usize>>();
    for (index, value) in nums.iter().enumerate() {
        if let Some(&j) = nums_map.get(value) {
            if index != j {
                return vec![index as i32, j as i32];
            }
        }
    }
    unreachable!();
}
#[test]
fn test2_1() {
    let v2 = vec![2, 11, 7, 9, 15];
    let b = two_sum(v2, 9);
    assert_eq!(b, vec![0, 2]);
}
