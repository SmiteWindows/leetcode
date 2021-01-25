// https://leetcode-cn.com/problems/minimum-number-of-days-to-disconnect-island/
// Runtime: 16 ms
// Memory Usage: 2.2 MB
pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    if islands(&grid, n, m) != 1 {
        return 0;
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                if islands(&grid, n, m) > 1 {
                    return 1;
                }
                grid[i][j] = 1;
            }
        }
    }

    2
}

fn islands(grid: &[Vec<i32>], n: usize, m: usize) -> i32 {
    let mut visited = vec![vec![false; m]; n];
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] && grid[i][j] == 1 {
                res += 1;
                visited[i][j] = true;
                dfs(i, j, &mut visited, grid, n, m);
            }
        }
    }
    res
}

fn dfs(i: usize, j: usize, visited: &mut Vec<Vec<bool>>, grid: &[Vec<i32>], n: usize, m: usize) {
    if i > 0 && grid[i - 1][j] == 1 && !visited[i - 1][j] {
        visited[i - 1][j] = true;
        dfs(i - 1, j, visited, grid, n, m);
    }
    if j > 0 && grid[i][j - 1] == 1 && !visited[i][j - 1] {
        visited[i][j - 1] = true;
        dfs(i, j - 1, visited, grid, n, m);
    }
    if i + 1 < n && grid[i + 1][j] == 1 && !visited[i + 1][j] {
        visited[i + 1][j] = true;
        dfs(i + 1, j, visited, grid, n, m);
    }
    if j + 1 < m && grid[i][j + 1] == 1 && !visited[i][j + 1] {
        visited[i][j + 1] = true;
        dfs(i, j + 1, visited, grid, n, m);
    }
}
// greedy
#[test]
fn test1_1568() {
    use leetcode_prelude::vec2;
    assert_eq!(min_days(vec2![[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]), 2);
    assert_eq!(min_days(vec2![[1, 1]]), 2);
    assert_eq!(min_days(vec2![[1, 0, 1, 0]]), 0);
    assert_eq!(
        min_days(vec2![
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1]
        ]),
        1
    );
    assert_eq!(
        min_days(vec2![
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1]
        ]),
        2
    );
}
