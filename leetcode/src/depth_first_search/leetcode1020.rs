// https://leetcode-cn.com/problems/number-of-enclaves/
// Runtime: 16 ms
// Memory Usage: 2.4 MB
pub fn num_enclaves(mut a: Vec<Vec<i32>>) -> i32 {
    let n = a.len();
    let m = a[0].len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 1 && (i == 0 || j == 0 || i + 1 == n || j + 1 == m) {
                dfs(i, j, &mut a, n, m);
            }
        }
    }
    for ai in a.iter().take(n) {
        for &aij in ai.iter().take(m) {
            if aij == 1 {
                res += 1;
            }
        }
    }
    res
}

fn dfs(i: usize, j: usize, a: &mut Vec<Vec<i32>>, n: usize, m: usize) {
    a[i][j] = 0;
    if i > 0 && a[i - 1][j] == 1 {
        dfs(i - 1, j, a, n, m);
    }
    if j > 0 && a[i][j - 1] == 1 {
        dfs(i, j - 1, a, n, m);
    }
    if i + 1 < n && a[i + 1][j] == 1 {
        dfs(i + 1, j, a, n, m);
    }
    if j + 1 < m && a[i][j + 1] == 1 {
        dfs(i, j + 1, a, n, m);
    }
}
// depth_first_search
#[test]
fn test1_1020() {
    use leetcode_prelude::vec2;
    assert_eq!(
        num_enclaves(vec2![
            [0, 0, 0, 0],
            [1, 0, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]),
        3
    );
    assert_eq!(
        num_enclaves(vec2![
            [0, 1, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 0]
        ]),
        0
    );
}
