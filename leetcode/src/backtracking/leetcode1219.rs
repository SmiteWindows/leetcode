// https://leetcode-cn.com/problems/path-with-maximum-gold/
// Runtime: 16 ms
// Memory Usage: 2.1 MB
pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let n = grid.len();
    let m = grid[0].len();
    let mut sum = 0;
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            dfs(i, j, &mut sum, &mut res, &mut grid, n, m);
        }
    }
    res
}

fn dfs(
    i: usize,
    j: usize,
    sum: &mut i32,
    max: &mut i32,
    grid: &mut Vec<Vec<i32>>,
    n: usize,
    m: usize,
) {
    if grid[i][j] == 0 {
        return;
    }
    let val = grid[i][j];
    *sum += val;
    *max = (*max).max(*sum);
    grid[i][j] = 0;
    if i > 0 {
        dfs(i - 1, j, sum, max, grid, n, m);
    }
    if j > 0 {
        dfs(i, j - 1, sum, max, grid, n, m);
    }
    if i + 1 < n {
        dfs(i + 1, j, sum, max, grid, n, m);
    }
    if j + 1 < m {
        dfs(i, j + 1, sum, max, grid, n, m);
    }
    grid[i][j] = val;
    *sum -= grid[i][j];
}
// backtracking
#[test]
fn test1_1219() {
    use leetcode_prelude::vec2;
    assert_eq!(get_maximum_gold(vec2![[0, 6, 0], [5, 8, 7], [0, 9, 0]]), 24);
    assert_eq!(
        get_maximum_gold(vec2![
            [1, 0, 7],
            [2, 0, 6],
            [3, 4, 5],
            [0, 3, 0],
            [9, 0, 20]
        ]),
        28
    );
}
