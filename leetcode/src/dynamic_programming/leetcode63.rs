// https://leetcode-cn.com/problems/unique-paths-ii/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let n = obstacle_grid.len();
    let m = obstacle_grid[0].len();
    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..m {
            if obstacle_grid[i][j] == 0 {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            } else {
                dp[i][j] = 0;
            }
        }
    }
    dp[n - 1][m - 1]
}
// array dynamic_programming
#[test]
fn test1_63() {
    assert_eq!(
        unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
}
