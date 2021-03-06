// https://leetcode-cn.com/problems/delete-operation-for-two-strings/
// Runtime: 4 ms
// Memory Usage: 3.8 MB
pub fn min_distance(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let m = word2.len();
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, &w1i) in w1.iter().enumerate().take(n) {
        for (j, &w2j) in w2.iter().enumerate().take(m) {
            if w1i == w2j {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    (n + m - dp[n][m] * 2) as i32
}
// string
#[test]
fn test1_583() {
    assert_eq!(min_distance("sea".to_string(), "eat".to_string()), 2);
}
