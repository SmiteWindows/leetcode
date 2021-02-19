// https://leetcode-cn.com/problems/range-sum-of-sorted-subarray-sums/
// Runtime: 52 ms
// Memory Usage: 4.1 MB
const MOD: i64 = 1_000_000_007;
pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let mut sums = Vec::new();
    for (i, &num) in nums.iter().enumerate() {
        let k = sums.len();
        for j in 0..i {
            sums.push(sums[k - 1 - j] + num);
        }
        sums.push(num);
    }
    sums.sort_unstable();
    let mut res = 0;
    let start = left as usize - 1;
    let end = right as usize;
    for &sum in sums.iter().take(end).skip(start) {
        res += sum as i64;
        res %= MOD;
    }
    res as i32
}
// array sort
#[test]
fn test1_1508() {
    assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
    assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
}
