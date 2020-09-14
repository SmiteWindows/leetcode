// https://leetcode-cn.com/problems/max-increase-to-keep-city-skyline/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut row = vec![0; n];
    let mut col = vec![0; m];
    let mut res = 0;
    for i in 0..n {
        for (j, cj) in col.iter_mut().enumerate().take(m) {
            row[i] = row[i].max(grid[i][j]);
            *cj = (*cj).max(grid[i][j]);
        }
    }
    for i in 0..n {
        for (j, &cj) in col.iter().enumerate().take(m) {
            res += row[i].min(cj) - grid[i][j];
        }
    }
    res
}
#[test]
fn test807() {
    use leetcode_prelude::vec2;
    assert_eq!(
        max_increase_keeping_skyline(vec2![
            [3, 0, 8, 4],
            [2, 4, 5, 7],
            [9, 2, 6, 3],
            [0, 3, 1, 0]
        ]),
        35
    );
}
