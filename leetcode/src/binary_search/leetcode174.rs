// https://leetcode.com/problems/dungeon-game/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let n = dungeon.len();
    let m = dungeon[0].len();
    let mut dp = vec![vec![0; m]; n];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if i + 1 < n && j + 1 < m {
                dp[i][j] = 1.max(-dungeon[i][j] + dp[i + 1][j].min(dp[i][j + 1]));
                continue;
            }
            if i + 1 < n {
                dp[i][j] = 1.max(-dungeon[i][j] + dp[i + 1][j]);
                continue;
            }
            if j + 1 < m {
                dp[i][j] = 1.max(-dungeon[i][j] + dp[i][j + 1]);
                continue;
            }
            dp[i][j] = 1.max(-dungeon[i][j] + 1);
        }
    }
    dp[0][0]
}
// binary_search dynamic_programming
#[test]
fn test1_174() {
    assert_eq!(
        calculate_minimum_hp(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]]),
        7
    );
}
