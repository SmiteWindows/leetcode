// https://leetcode-cn.com/problems/continuous-subarray-sum/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let mut sum = 0;
    let mut pre = 0;
    let mut hs = HashSet::new();
    for num in nums.iter().take(n) {
        sum += num;
        let cur = if k == 0 { sum } else { sum % k };
        if hs.contains(&cur) {
            return true;
        }
        hs.insert(pre);
        pre = cur;
    }
    false
}
// math dynamic_programming
#[test]
fn test2_523() {
    assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
}
