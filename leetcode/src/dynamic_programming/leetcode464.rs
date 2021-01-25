// https://leetcode-cn.com/problems/can-i-win/
// Runtime: 36 ms
// Memory Usage: 3.8 MB
use std::collections::HashMap;
pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    let nums = (1..=max_choosable_integer).collect::<Vec<_>>();
    let n = nums.len();
    if desired_total > nums.iter().sum::<i32>() {
        return false;
    }
    let mut memo = HashMap::new();
    dp(desired_total, (1 << n) - 1, &mut memo, &nums, n)
}

fn dp(total: i32, bitset: u32, memo: &mut HashMap<u32, bool>, nums: &[i32], n: usize) -> bool {
    if let Some(&res) = memo.get(&bitset) {
        return res;
    }
    let mut res = false;
    for i in 0..n {
        if res {
            break;
        }
        if 1 << i & bitset > 0 {
            if total - nums[i] <= 0 {
                res = true;
            }
            res |= !dp(total - nums[i], bitset & !(1 << i), memo, nums, n);
        }
    }
    memo.insert(bitset, res);
    res
}
// dynamic_programming minimax
#[test]
fn test2_464() {
    assert_eq!(can_i_win(10, 11), false);
}
