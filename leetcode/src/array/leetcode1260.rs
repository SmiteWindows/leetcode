// https://leetcode-cn.com/problems/shift-2d-grid/
// Runtime: 8 ms
// Memory Usage: 2 MB
pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut arr = Vec::new();
    for row in grid.iter().take(m) {
        for &col in row.iter().take(n) {
            arr.push(col);
        }
    }
    let s = n * m;
    let mut k = s - (k as usize) % s;
    for row in grid.iter_mut().take(m) {
        for col in row.iter_mut().take(n) {
            *col = arr[k % s];
            k += 1;
        }
    }
    grid
}
// array
#[test]
fn test1_1260() {
    use leetcode_prelude::vec2;
    assert_eq!(
        shift_grid(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 1),
        vec2![[9, 1, 2], [3, 4, 5], [6, 7, 8]]
    );
    assert_eq!(
        shift_grid(
            vec2![[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]],
            4
        ),
        vec2![[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]
    );
    assert_eq!(
        shift_grid(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 9),
        vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    );
}
