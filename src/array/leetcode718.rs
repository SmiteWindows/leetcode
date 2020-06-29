// https://leetcode.com/problems/maximum-length-of-repeated-subarray/
// Runtime: 32 ms
// Memory Usage: 5.8 MB
pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
                res = res.max(dp[i + 1][j + 1]);
            }
        }
    }
    res
}
// binary_search hash_table array dynamic_programming
#[test]
fn test2_718() {
    assert_eq!(find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]), 3);
}
