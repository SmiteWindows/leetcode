// https://leetcode.com/problems/longest-increasing-path-in-a-matrix
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    if n == 0 {
        return 0;
    }
    let m = matrix[0].len();
    let mut memo = vec![vec![0; m]; n];
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if memo[i][j] == 0 {
                memo[i][j] = dfs(i, j, &mut memo, &matrix);
                res = res.max(memo[i][j]);
            }
        }
    }
    res
}

fn dfs(i: usize, j: usize, memo: &mut Vec<Vec<i32>>, matrix: &[Vec<i32>]) -> i32 {
    if memo[i][j] != 0 {
        return memo[i][j];
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = 1;
    if i > 0 && matrix[i - 1][j] > matrix[i][j] {
        res = res.max(dfs(i - 1, j, memo, matrix) + 1);
    }
    if j > 0 && matrix[i][j - 1] > matrix[i][j] {
        res = res.max(dfs(i, j - 1, memo, matrix) + 1);
    }
    if i + 1 < n && matrix[i + 1][j] > matrix[i][j] {
        res = res.max(dfs(i + 1, j, memo, matrix) + 1);
    }
    if j + 1 < m && matrix[i][j + 1] > matrix[i][j] {
        res = res.max(dfs(i, j + 1, memo, matrix) + 1);
    }
    memo[i][j] = res;
    res
}
// depth_first_search topological_sort memoization
#[test]
fn test3_329() {
    assert_eq!(
        longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    );
    assert_eq!(
        longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
        4
    );
}
