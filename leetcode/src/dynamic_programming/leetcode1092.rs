// https://leetcode-cn.com/problems/shortest-common-supersequence/
// Runtime: 4 ms
// Memory Usage: 33.3 MB
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let n = s1.len();
    let m = s2.len();
    let mut dp = vec![vec![(' ', 0, std::usize::MAX, std::usize::MAX); m + 1]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = (s1[i], i + 1, i, 0);
    }
    for (j, &s2j) in s2.iter().enumerate().take(m) {
        dp[0][j + 1] = (s2j, j + 1, 0, j);
    }
    for i in 0..n {
        for (j, &s2j) in s2.iter().enumerate().take(m) {
            if s1[i] == s2[j] {
                dp[i + 1][j + 1] = (s1[i], dp[i][j].1 + 1, i, j);
            } else if dp[i][j + 1].1 < dp[i + 1][j].1 {
                dp[i + 1][j + 1] = (s1[i], dp[i][j + 1].1 + 1, i, j + 1);
            } else {
                dp[i + 1][j + 1] = (s2j, dp[i + 1][j].1 + 1, i + 1, j);
            }
        }
    }
    let mut path = Vec::new();
    let mut i = n;
    let mut j = m;
    while dp[i][j].0 != ' ' {
        path.push(dp[i][j].0);
        let next = dp[i][j];
        i = next.2;
        j = next.3;
    }
    path.into_iter().rev().collect()
}
// dynamic_programming
#[test]
fn test1_1092() {
    assert_eq!(
        shortest_common_supersequence("abac".to_string(), "cab".to_string()),
        "cabac".to_string()
    );
}
