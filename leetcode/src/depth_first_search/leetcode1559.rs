// https://leetcode.com/problems/detect-cycles-in-2d-grid/
// Runtime: 88 ms
// Memory Usage: 33.3 MB
#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            let c = grid[i][j];
            if dfs(
                i,
                j,
                0,
                usize::MAX,
                usize::MAX,
                &mut visited,
                &grid,
                c,
                n,
                m,
            ) {
                return true;
            }
        }
    }
    false
}

fn dfs(
    i: usize,
    j: usize,
    dist: usize,
    pi: usize,
    pj: usize,
    visited: &mut Vec<Vec<bool>>,
    grid: &[Vec<char>],
    c: char,
    n: usize,
    m: usize,
) -> bool {
    if dist >= 4 && visited[i][j] {
        return true;
    }
    if visited[i][j] {
        return false;
    }
    visited[i][j] = true;
    if i > 0
        && grid[i - 1][j] == c
        && i - 1 != pi
        && dfs(i - 1, j, dist + 1, i, j, visited, grid, c, n, m)
    {
        return true;
    }
    if j > 0
        && grid[i][j - 1] == c
        && j - 1 != pj
        && dfs(i, j - 1, dist + 1, i, j, visited, grid, c, n, m)
    {
        return true;
    }
    if i + 1 < n
        && grid[i + 1][j] == c
        && i + 1 != pi
        && dfs(i + 1, j, dist + 1, i, j, visited, grid, c, n, m)
    {
        return true;
    }
    if j + 1 < m
        && grid[i][j + 1] == c
        && j + 1 != pj
        && dfs(i, j + 1, dist + 1, i, j, visited, grid, c, n, m)
    {
        return true;
    }
    false
}
// depth_first_search
#[test]
fn test1_1559() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        contains_cycle(vec2_char![
            ['a', 'a', 'a', 'a'],
            ['a', 'b', 'b', 'a'],
            ['a', 'b', 'b', 'a'],
            ['a', 'a', 'a', 'a']
        ]),
        true
    );
    assert_eq!(
        contains_cycle(vec2_char![
            ['c', 'c', 'c', 'a'],
            ['c', 'd', 'c', 'c'],
            ['c', 'c', 'e', 'c'],
            ['f', 'c', 'c', 'c']
        ]),
        true
    );
    assert_eq!(
        contains_cycle(vec2_char![
            ['a', 'b', 'b'],
            ['b', 'z', 'b'],
            ['b', 'b', 'a']
        ]),
        false
    );
}
