// https://leetcode-cn.com/problems/longest-common-subsequence/
// Runtime: 4 ms
// Memory Usage: 9.8 MB
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let s1 = text1.chars().collect::<Vec<char>>();
    let s2 = text2.chars().collect::<Vec<char>>();
    let n1 = s1.len();
    let n2 = s2.len();
    let mut dp = vec![vec![0_usize; n2 + 1]; n1 + 1];
    for (i, &s1i) in s1.iter().enumerate().take(n1) {
        for (j, &s2j) in s2.iter().enumerate().take(n2) {
            if s1i == s2j {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    dp[n1][n2] as i32
}
// dynamic_programming
#[test]
fn test1_1143() {
    assert_eq!(
        longest_common_subsequence("abcde".to_string(), "ace".to_string()),
        3
    );
    assert_eq!(
        longest_common_subsequence("abc".to_string(), "abc".to_string()),
        3
    );
    assert_eq!(
        longest_common_subsequence("abc".to_string(), "def".to_string()),
        0
    );
}
