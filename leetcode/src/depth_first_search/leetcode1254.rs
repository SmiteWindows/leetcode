// https://leetcode-cn.com/problems/number-of-closed-islands/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut res = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 && dfs(i, j, &mut grid, n, m) {
                res += 1;
            }
        }
    }
    res
}

fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<i32>>, n: usize, m: usize) -> bool {
    grid[i][j] = 1;
    let top = if i > 0 {
        if grid[i - 1][j] == 1 {
            true
        } else {
            dfs(i - 1, j, grid, n, m)
        }
    } else {
        false
    };
    let left = if j > 0 {
        if grid[i][j - 1] == 1 {
            true
        } else {
            dfs(i, j - 1, grid, n, m)
        }
    } else {
        false
    };
    let bottom = if i + 1 < n {
        if grid[i + 1][j] == 1 {
            true
        } else {
            dfs(i + 1, j, grid, n, m)
        }
    } else {
        false
    };
    let right = if j + 1 < m {
        if grid[i][j + 1] == 1 {
            true
        } else {
            dfs(i, j + 1, grid, n, m)
        }
    } else {
        false
    };
    top && left && bottom && right
}
// depth_first_search
#[test]
fn test1_1254() {
    use leetcode_prelude::vec2;
    assert_eq!(
        closed_island(vec2![
            [1, 1, 1, 1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 1, 0],
            [1, 0, 1, 0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0]
        ]),
        2
    );
    assert_eq!(
        closed_island(vec2![[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]]),
        1
    );
    assert_eq!(
        closed_island(vec2![
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1]
        ]),
        2
    );
}
