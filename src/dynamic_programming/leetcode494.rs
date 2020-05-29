// https://leetcode.com/problems/target-sum/
use std::collections::HashMap;
pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    let n = nums.len();
    let mut memo = HashMap::new();
    dfs(0, s, &mut memo, &nums, n)
}

fn dfs(
    start: usize,
    sum: i32,
    memo: &mut HashMap<(usize, i32), i32>,
    nums: &[i32],
    n: usize,
) -> i32 {
    if start == n {
        if sum == 0 {
            1
        } else {
            0
        }
    } else {
        if let Some(&res) = memo.get(&(start, sum)) {
            res
        } else {
            let a = dfs(start + 1, sum + nums[start], memo, nums, n);
            let b = dfs(start + 1, sum - nums[start], memo, nums, n);
            let res = a + b;
            memo.insert((start, sum), res);
            res
        }
    }
}
// dynamic_programming depth_first_search
#[test]
fn test1_494() {
    assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
}
