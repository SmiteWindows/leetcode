// https://leetcode-cn.com/problems/knight-dialer/
// Runtime: 16 ms
// Memory Usage: 2.1 MB
pub fn knight_dialer(n: i32) -> i32 {
    let max = 1_000_000_007;
    let n = n as usize;
    let mut dp: [[usize; 10]; 2] = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let map = vec![
        vec![4, 6],
        vec![6, 8],
        vec![7, 9],
        vec![4, 8],
        vec![0, 3, 9],
        vec![],
        vec![0, 1, 7],
        vec![2, 6],
        vec![1, 3],
        vec![2, 4],
    ];
    let mut res = 10;
    for i in 1..n {
        res = 0;
        for (j, mj) in map.iter().enumerate().take(10) {
            let mut sum = 0;
            for &k in mj {
                sum += dp[(i - 1) % 2][k];
                sum %= max;
            }
            dp[i % 2][j] = sum;
            res += dp[i % 2][j];
            res %= max;
        }
    }
    res as i32
}
// dynamic_programming
#[test]
fn test1_935() {
    assert_eq!(knight_dialer(1), 10);
    assert_eq!(knight_dialer(2), 20);
    assert_eq!(knight_dialer(3), 46);
}
