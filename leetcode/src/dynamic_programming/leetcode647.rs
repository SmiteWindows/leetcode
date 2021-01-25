// https://leetcode-cn.com/problems/palindromic-substrings/
// Runtime: 8 ms
// Memory Usage: 3.1 MB
pub fn count_substrings(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut res = 0;
    let mut dp = vec![vec![false; n + 1]; n + 1];
    for i in (0..n).rev() {
        for j in i..n {
            if j == i || j == i + 1 && s[i] == s[j] || j > i + 1 && s[i] == s[j] && dp[i + 1][j - 1]
            {
                dp[i][j] = true;
                res += 1;
            }
        }
    }
    res
}
// dynamic_programming string
#[test]
fn test1_647() {
    assert_eq!(count_substrings("abc".to_string()), 3);
    assert_eq!(count_substrings("aaa".to_string()), 6);
}
