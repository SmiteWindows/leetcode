// https://leetcode-cn.com/problems/minimum-path-sum/
// Runtime: 0 ms
// Memory Usage: 2.5 MB
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = grid[0][0];
    for j in 1..m {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }
    for i in 1..n {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }
    for i in 1..n {
        for j in 1..m {
            dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
        }
    }
    dp[n - 1][m - 1]
}
// array dynamic_programming
#[test]
fn test1_64() {
    use leetcode_prelude::vec2;
    assert_eq!(min_path_sum(vec2![[1, 3, 1], [1, 5, 1], [4, 2, 1]]), 7);
}
