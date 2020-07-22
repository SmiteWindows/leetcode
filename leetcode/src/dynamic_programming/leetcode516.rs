// https://leetcode.com/problems/longest-palindromic-subsequence/
// Runtime: 20 ms
// Memory Usage: 5.8 MB
pub fn longest_palindrome_subseq(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut dp = vec![vec![0; n + 1]; n];
    for (i, dpi) in dp.iter_mut().enumerate().take(n) {
        dpi[i + 1] = 1;
    }
    for w in 2..=n {
        for i in 0..=n - w {
            let j = i + w;
            if s[i] == s[j - 1] {
                dp[i][j] = 2 + dp[i + 1][j - 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n]
}
// dynamic_programming
#[test]
fn test1_516() {
    assert_eq!(longest_palindrome_subseq(String::from("bbbab")), 4);
    assert_eq!(longest_palindrome_subseq(String::from("cbbd")), 2);
}
