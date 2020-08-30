// https://leetcode-cn.com/problems/01-matrix/
// Runtime: 32 ms
// Memory Usage: 3.5 MB
use std::collections::VecDeque;
pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matrix = matrix;
    let n = matrix.len();
    let m = matrix[0].len();
    let mut visited = vec![vec![false; m]; n];
    let mut queue = VecDeque::new();
    for (i, mi) in matrix.iter().enumerate().take(n) {
        for (j, &mij) in mi.iter().enumerate().take(m) {
            if mij == 0 {
                queue.push_back((i, j, 0));
            }
        }
    }
    while let Some((i, j, d)) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        matrix[i][j] = d;
        if i > 0 {
            queue.push_back((i - 1, j, d + 1));
        }
        if j > 0 {
            queue.push_back((i, j - 1, d + 1));
        }
        if i + 1 < n {
            queue.push_back((i + 1, j, d + 1));
        }
        if j + 1 < m {
            queue.push_back((i, j + 1, d + 1));
        }
    }
    matrix
}
// depth_first_search breadth_first_search
#[test]
fn test1_542() {
    assert_eq!(
        update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    );
    assert_eq!(
        update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
    );
}
