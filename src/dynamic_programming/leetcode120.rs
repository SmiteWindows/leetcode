// https://leetcode.com/problems/triangle/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut dp = vec![vec![0; n + 1]; 2];
    for i in (0..n).rev() {
        for j in 0..=i {
            dp[i % 2][j] = triangle[i][j] + dp[(i + 1) % 2][j].min(dp[(i + 1) % 2][j + 1]);
        }
    }
    dp[0][0]
}
// dynamic_programming array
#[test]
fn test1_120() {
    assert_eq!(
        minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
}
