// https://leetcode-cn.com/problems/surface-area-of-3d-shapes/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    for gi in grid.iter().take(n) {
        for &gj in gi.iter().take(n) {
            if gj > 0 {
                res += 2 + 4 * gj;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if i > 0 {
                res -= 2 * grid[i][j].min(grid[i - 1][j]);
            }
            if j > 0 {
                res -= 2 * grid[i][j].min(grid[i][j - 1]);
            }
        }
    }
    res
}
// math geometry
#[test]
fn test1_892() {
    assert_eq!(surface_area(vec![vec![2]]), 10);
    assert_eq!(surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(surface_area(vec![vec![1, 0], vec![0, 2]]), 16);
    assert_eq!(
        surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        32
    );
    assert_eq!(
        surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
        46
    );
}
