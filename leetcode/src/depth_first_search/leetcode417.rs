// https://leetcode-cn.com/problems/pacific-atlantic-water-flow/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let n = matrix.len();
    if n == 0 {
        return res;
    }
    let m = matrix[0].len();
    let mut visited = vec![vec![vec![false; m]; n]; 2];
    for i in 0..n {
        dfs(i, 0, &matrix, &mut visited[0]);
    }
    for i in 0..n {
        dfs(i, m - 1, &matrix, &mut visited[1]);
    }
    for j in 0..m {
        dfs(0, j, &matrix, &mut visited[0])
    }
    for j in 0..m {
        dfs(n - 1, j, &matrix, &mut visited[1])
    }
    for i in 0..n {
        for j in 0..m {
            if visited[0][i][j] && visited[1][i][j] {
                res.push(vec![i as i32, j as i32]);
            }
        }
    }
    res
}

fn dfs(i: usize, j: usize, matrix: &[Vec<i32>], visited: &mut Vec<Vec<bool>>) {
    if !visited[i][j] {
        visited[i][j] = true;
        if i > 0 && matrix[i - 1][j] >= matrix[i][j] {
            dfs(i - 1, j, matrix, visited);
        }
        if i + 1 < matrix.len() && matrix[i + 1][j] >= matrix[i][j] {
            dfs(i + 1, j, matrix, visited);
        }
        if j > 0 && matrix[i][j - 1] >= matrix[i][j] {
            dfs(i, j - 1, matrix, visited);
        }
        if j + 1 < matrix[i].len() && matrix[i][j + 1] >= matrix[i][j] {
            dfs(i, j + 1, matrix, visited);
        }
    }
}
// depth_first_search breadth_first_search
#[test]
fn test1_417() {
    use leetcode_prelude::vec2;
    assert_eq!(
        pacific_atlantic(vec2![
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4]
        ]),
        vec2![[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
    );
}
