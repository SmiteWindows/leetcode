// https://leetcode-cn.com/problems/minimum-falling-path-sum-ii/
// Runtime: 56 ms
// Memory Usage: 2.5 MB
pub fn min_falling_path_sum(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let m = arr[0].len();
    let mut dp = vec![vec![0; m]; n + 1];
    for i in 0..n {
        for j in 0..m {
            let mut min = i32::MAX;
            for k in 0..m {
                if k != j {
                    min = min.min(dp[i][k]);
                }
            }
            dp[i + 1][j] = arr[i][j] + min;
        }
    }
    *dp[n].iter().min().unwrap()
}
// dynamic_programming
#[test]
fn test1_1289() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_falling_path_sum(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
        13
    );
}
