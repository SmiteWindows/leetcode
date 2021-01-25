// https://leetcode-cn.com/problems/target-sum/
// Runtime: 40 ms
// Memory Usage: 2.5 MB
use std::collections::HashMap;
pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    let n = nums.len();
    let mut memo = HashMap::new();
    dp(n, s, &mut memo, &nums, n)
}

fn dp(end: usize, sum: i32, memo: &mut HashMap<(usize, i32), i32>, nums: &[i32], n: usize) -> i32 {
    if end == 0 {
        if sum == 0 {
            1
        } else {
            0
        }
    } else {
        if let Some(&res) = memo.get(&(end, sum)) {
            return res;
        }
        let a = dp(end - 1, sum + nums[end - 1], memo, nums, n);
        let b = dp(end - 1, sum - nums[end - 1], memo, nums, n);
        let res = a + b;
        memo.insert((end, sum), res);
        res
    }
}
// dynamic_programming depth_first_search
#[test]
fn test2_494() {
    assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
}
