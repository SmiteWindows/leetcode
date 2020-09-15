// https://leetcode-cn.com/problems/coloring-a-border/
#![allow(clippy::too_many_arguments)]
#![allow(clippy::many_single_char_names)]
// Runtime: 8 ms
// Memory Usage: 2 MB
pub fn color_border(mut grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let r0 = r0 as usize;
    let c0 = c0 as usize;
    let c_color = grid[r0][c0];
    let b_color = color;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    dfs(r0, c0, &mut visited, &mut grid, b_color, c_color, n, m);
    grid
}

fn dfs(
    i: usize,
    j: usize,
    visited: &mut [Vec<bool>],
    grid: &mut [Vec<i32>],
    b_color: i32,
    c_color: i32,
    n: usize,
    m: usize,
) {
    visited[i][j] = true;
    let top = if i == 0 {
        true
    } else if grid[i - 1][j] != c_color {
        !visited[i - 1][j]
    } else {
        if !visited[i - 1][j] {
            dfs(i - 1, j, visited, grid, b_color, c_color, n, m);
        }
        false
    };
    let left = if j == 0 {
        true
    } else if grid[i][j - 1] != c_color {
        !visited[i][j - 1]
    } else {
        if !visited[i][j - 1] {
            dfs(i, j - 1, visited, grid, b_color, c_color, n, m);
        }
        false
    };
    let down = if i + 1 == n {
        true
    } else if grid[i + 1][j] != c_color {
        !visited[i + 1][j]
    } else {
        if !visited[i + 1][j] {
            dfs(i + 1, j, visited, grid, b_color, c_color, n, m);
        }
        false
    };
    let right = if j + 1 == m {
        true
    } else if grid[i][j + 1] != c_color {
        !visited[i][j + 1]
    } else {
        if !visited[i][j + 1] {
            dfs(i, j + 1, visited, grid, b_color, c_color, n, m);
        }
        false
    };
    if top || left || down || right {
        grid[i][j] = b_color;
    }
}
// depth_first_search
#[test]
fn test1_1034() {
    use leetcode_prelude::vec2;
    assert_eq!(
        color_border(vec2![[1, 1], [1, 2]], 0, 0, 3),
        vec2![[3, 3], [3, 2]]
    );
    assert_eq!(
        color_border(vec2![[1, 2, 2], [2, 3, 2]], 0, 1, 3),
        vec2![[1, 3, 3], [2, 3, 3]]
    );
    assert_eq!(
        color_border(vec2![[1, 1, 1], [1, 1, 1], [1, 1, 1]], 1, 1, 2),
        vec2![[2, 2, 2], [2, 1, 2], [2, 2, 2]]
    );
}
