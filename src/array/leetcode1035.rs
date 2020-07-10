// https://leetcode.com/problems/uncrossed-lines/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, &ai) in a.iter().enumerate().take(n) {
        for (j, &bj) in b.iter().enumerate().take(m) {
            if ai == bj {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    dp[n][m]
}
// array
#[test]
fn test1_1035() {
    assert_eq!(max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
    assert_eq!(
        max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
    assert_eq!(
        max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}
