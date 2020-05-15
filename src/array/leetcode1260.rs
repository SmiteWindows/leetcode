// https://leetcode.com/problems/shift-2d-grid/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut grid = grid;
    let m = grid.len();
    let n = grid[0].len();
    let mut arr = vec![];
    for i in 0..m {
        for j in 0..n {
            arr.push(grid[i][j]);
        }
    }
    let s = n * m;
    let mut k = s - (k as usize) % s;
    for i in 0..m {
        for j in 0..n {
            grid[i][j] = arr[k % s];
            k += 1;
        }
    }
    grid
}
// array
#[test]
fn test1_1260() {
    assert_eq!(
        shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
    );
    assert_eq!(
        shift_grid(
            vec![
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
                vec![12, 0, 21, 13]
            ],
            4
        ),
        vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10]
        ]
    );
    assert_eq!(
        shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    );
}
