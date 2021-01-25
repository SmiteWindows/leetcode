// https://leetcode.com/problems/maximum-sum-obtained-of-any-permutation/
// Runtime: 64 ms
// Memory Usage: 6.6 MB
const MOD: i64 = 1_000_000_007;
pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut count = vec![0; n + 1];
    for r in requests {
        count[r[0] as usize] += 1;
        count[r[1] as usize + 1] -= 1;
    }
    count.pop();
    let mut prev = 0;
    for ci in count.iter_mut().take(n) {
        prev += *ci;
        *ci = prev;
    }
    count.sort_unstable();
    nums.sort_unstable();
    let mut res = 0;
    for i in 0..n {
        res += nums[i] as i64 * count[i] as i64;
        res %= MOD;
    }
    res as i32
}
// greedy
#[test]
fn test1_1589() {
    use leetcode_prelude::vec2;
    assert_eq!(
        max_sum_range_query(vec![1, 2, 3, 4, 5], vec2![[1, 3], [0, 1]]),
        19
    );
    assert_eq!(
        max_sum_range_query(vec![1, 2, 3, 4, 5, 6], vec2![[0, 1]]),
        11
    );
    assert_eq!(
        max_sum_range_query(vec![1, 2, 3, 4, 5, 10], vec2![[0, 2], [1, 3], [1, 1]]),
        47
    );
}
