// https://leetcode.com/problems/unique-paths/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut a = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if i == 1 && j == 1 {
                a[i][j] = 1;
            } else {
                a[i][j] = a[i - 1][j] + a[i][j - 1];
            }
        }
    }
    a[n][m]
}
// array dynamic_programming
#[test]
fn test2_62() {
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(7, 3), 28);
}
