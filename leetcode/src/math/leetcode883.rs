// https://leetcode.com/problems/projection-area-of-3d-shapes/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut x = vec![0; n];
    let mut y = vec![0; m];
    let mut sum_z = 0;
    for (i, xi) in x.iter_mut().enumerate().take(n) {
        for (j, yj) in y.iter_mut().enumerate().take(m) {
            if grid[i][j] != 0 {
                sum_z += 1;
            }
            *xi = (*xi).max(grid[i][j]);
            *yj = (*yj).max(grid[i][j]);
        }
    }
    let sum_x = x.iter().sum::<i32>();
    let sum_y = y.iter().sum::<i32>();
    sum_x + sum_y + sum_z
}
// math
#[test]
fn test1_883() {
    assert_eq!(projection_area(vec![vec![2]]), 5);
    assert_eq!(projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    assert_eq!(
        projection_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        14
    );
    assert_eq!(
        projection_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
        21
    );
}
