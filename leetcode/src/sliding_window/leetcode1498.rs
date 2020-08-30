// https://leetcode-cn.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
// Runtime: 32 ms
// Memory Usage: 3.1 MB
const MOD: i32 = 1_000_000_007;
pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = 0;
    let n = nums.len();
    let mut power = vec![];
    let mut prev = 1;
    for _ in 0..n {
        power.push(prev);
        prev *= 2;
        prev %= MOD;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l <= r {
        if nums[l] + nums[r] <= target {
            res += power[r - l];
            res %= MOD;
            l += 1;
        } else {
            if r == 0 {
                break;
            }
            r -= 1;
        }
    }
    res
}
// sliding_window sort
#[test]
fn test1_1498() {
    assert_eq!(num_subseq(vec![3, 5, 6, 7], 9), 4);
    assert_eq!(num_subseq(vec![3, 3, 6, 8], 10), 6);
    assert_eq!(num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
    assert_eq!(num_subseq(vec![5, 2, 4, 1, 7, 6, 8], 16), 127);
}
