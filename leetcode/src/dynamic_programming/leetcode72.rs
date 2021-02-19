// https://leetcode-cn.com/problems/edit-distance/
// Runtime: 0 ms
// Memory Usage: 3.7 MB
pub fn min_distance(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let m = word2.len();
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, dpi) in dp.iter_mut().enumerate().take(n + 1) {
        dpi[0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
            }
        }
    }
    dp[n][m] as i32
}
// dynamic_programming string
#[test]
fn test1_72() {
    assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    assert_eq!(
        min_distance("intention".to_string(), "execution".to_string()),
        5
    );
}
