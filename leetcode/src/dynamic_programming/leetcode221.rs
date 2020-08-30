// https://leetcode-cn.com/problems/maximal-square/
// Runtime: 4 ms
// Memory Usage: 4.7 MB
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max = 0;
    let n = matrix.len();
    if n == 0 {
        return 0;
    }
    let m = matrix[0].len();
    if m == 0 {
        return 0;
    }
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if matrix[i - 1][j - 1] == '1' {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1])) + 1;
                max = max.max(dp[i][j]);
            }
        }
    }
    max * max
}
// dynamic_programming
#[test]
fn test1_221() {
    assert_eq!(
        maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        4
    );
}
