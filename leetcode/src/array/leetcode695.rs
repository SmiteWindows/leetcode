// https://leetcode.com/problems/max-area-of-island/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let n = grid.len();
    let m = grid[0].len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            res = res.max(area(&mut grid, n, m, i, j));
        }
    }
    res
}

fn area(grid: &mut Vec<Vec<i32>>, n: usize, m: usize, row: usize, col: usize) -> i32 {
    if grid[row][col] <= 0 {
        0
    } else {
        grid[row][col] *= -1;
        let mut sum = 1;
        if row > 0 {
            sum += area(grid, n, m, row - 1, col);
        }
        if row + 1 < n {
            sum += area(grid, n, m, row + 1, col);
        }
        if col > 0 {
            sum += area(grid, n, m, row, col - 1);
        }
        if col + 1 < m {
            sum += area(grid, n, m, row, col + 1);
        }
        sum
    }
}
// depth_first_search array
#[test]
fn test2_695() {
    assert_eq!(
        max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
    assert_eq!(max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]), 0);
}
