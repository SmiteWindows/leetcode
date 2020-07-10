// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let s1 = s1.bytes().collect::<Vec<u8>>();
    let s2 = s2.bytes().collect::<Vec<u8>>();
    let sum = s1.iter().chain(s2.iter()).map(|&b| b as i32).sum::<i32>();
    let n = s1.len();
    let m = s2.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            if s1[i] == s2[j] {
                dp[i + 1][j + 1] = dp[i][j] + s1[i] as i32 + s2[j] as i32;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    sum - dp[n][m]
}
// dynamic_programming
#[test]
fn test1_712() {
    assert_eq!(
        minimum_delete_sum(String::from("sea"), String::from("eat")),
        231
    );
    assert_eq!(
        minimum_delete_sum(String::from("delete"), String::from("leet")),
        403
    );
}
