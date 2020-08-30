// https://leetcode-cn.com/problems/surrounded-regions/
// Runtime: 12 ms
// Memory Usage: 4.9 MB
pub fn solve(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    if n == 0 {
        return;
    }
    let m = board[0].len();
    let mut visited = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if (i == 0 || j == 0 || i == n - 1 || j == m - 1)
                && board[i][j] == 'O'
                && !visited[i][j]
            {
                dfs(i, j, &mut visited, &board, n, m);
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                board[i][j] = 'X';
            }
        }
    }
}

fn dfs(i: usize, j: usize, visited: &mut [Vec<bool>], board: &[Vec<char>], n: usize, m: usize) {
    visited[i][j] = true;
    if i > 0 && !visited[i - 1][j] && board[i - 1][j] == 'O' {
        dfs(i - 1, j, visited, board, n, m);
    }
    if j > 0 && !visited[i][j - 1] && board[i][j - 1] == 'O' {
        dfs(i, j - 1, visited, board, n, m);
    }
    if i + 1 < n && !visited[i + 1][j] && board[i + 1][j] == 'O' {
        dfs(i + 1, j, visited, board, n, m);
    }
    if j + 1 < m && !visited[i][j + 1] && board[i][j + 1] == 'O' {
        dfs(i, j + 1, visited, board, n, m);
    }
}
// depth_first_search breadth_first_search union_find
#[test]
fn test1_130() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let res = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    solve(&mut board);
    assert_eq!(board, res);
}
