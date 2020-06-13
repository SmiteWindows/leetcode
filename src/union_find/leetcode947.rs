// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
// Runtime: 4 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let n = stones.len();
    let mut row: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut col: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, stone) in stones.iter().enumerate().take(n) {
        let r = stone[0];
        let c = stone[1];
        row.entry(r).or_default().push(i);
        col.entry(c).or_default().push(i);
    }
    let mut visited = vec![false; n];
    let mut island = 0;
    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            dfs(i, &mut visited, &stones, &row, &col);
            island += 1;
        }
    }
    (n - island) as i32
}

fn dfs(
    u: usize,
    visited: &mut [bool],
    stones: &[Vec<i32>],
    row: &HashMap<i32, Vec<usize>>,
    col: &HashMap<i32, Vec<usize>>,
) {
    let r = stones[u][0];
    let c = stones[u][1];
    for &v in &row[&r] {
        if !visited[v] {
            visited[v] = true;
            dfs(v, visited, stones, row, col);
        }
    }
    for &v in &col[&c] {
        if !visited[v] {
            visited[v] = true;
            dfs(v, visited, stones, row, col);
        }
    }
}
// union_find depth_first_search
#[test]
fn test1_947() {
    assert_eq!(
        remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2]
        ]),
        5
    );
    assert_eq!(
        remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2]
        ]),
        3
    );
    assert_eq!(remove_stones(vec![vec![0, 0]]), 0);
}
