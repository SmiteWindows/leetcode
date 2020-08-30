// https://leetcode-cn.com/problems/minimum-time-to-collect-all-apples-in-a-tree/
// Runtime: 48 ms
// Memory Usage: 15.8 MB
use std::collections::HashSet;
pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    let n = n as usize;
    let mut graph = vec![HashSet::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[u].insert(v);
        graph[v].insert(u);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, &mut visited, &graph, &has_apple)
}

fn dfs(u: usize, visited: &mut Vec<bool>, graph: &[HashSet<usize>], has_apple: &[bool]) -> i32 {
    let mut res = 0;
    for &v in &graph[u] {
        if !visited[v] {
            visited[v] = true;
            res += dfs(v, visited, graph, has_apple);
        }
    }
    if u != 0 && (res != 0 || has_apple[u]) {
        res += 2;
    }
    res
}
// tree depth_first_search
#[test]
fn test1_1443() {
    assert_eq!(
        min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, true, true, false]
        ),
        8
    );
    assert_eq!(
        min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, false, true, false]
        ),
        6
    );
    assert_eq!(
        min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, false, false, false, false, false]
        ),
        0
    );
}
