// https://leetcode.com/problems/course-schedule-iv/
// Runtime: 60 ms
// Memory Usage: 3 MB
use std::collections::HashSet;
pub fn check_if_prerequisite(
    n: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let n = n as usize;
    let mut graph = vec![HashSet::new(); n];
    for edge in prerequisites {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].insert(v);
    }
    let mut closures = vec![HashSet::new(); n];
    for i in 0..n {
        let mut visited = vec![false; n];
        dfs(i, &mut visited, &mut closures, &graph, i);
    }
    queries
        .into_iter()
        .map(|v| closures[v[0] as usize].contains(&(v[1] as usize)))
        .collect()
}

fn dfs(
    u: usize,
    visited: &mut Vec<bool>,
    closures: &mut Vec<HashSet<usize>>,
    graph: &[HashSet<usize>],
    start: usize,
) {
    if !visited[u] {
        visited[u] = true;
        for &v in &graph[u] {
            closures[start].insert(v);
            dfs(v, visited, closures, graph, start);
        }
    }
}
// graph
#[test]
fn test1_1462() {
    assert_eq!(
        check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
        vec![false, true]
    );
    assert_eq!(
        check_if_prerequisite(2, vec![], vec![vec![1, 0], vec![0, 1]]),
        vec![false, false]
    );
    assert_eq!(
        check_if_prerequisite(
            3,
            vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            vec![vec![1, 0], vec![1, 2]]
        ),
        vec![true, true]
    );
    assert_eq!(
        check_if_prerequisite(
            3,
            vec![vec![1, 0], vec![2, 0]],
            vec![vec![0, 1], vec![2, 0]]
        ),
        vec![false, true]
    );
    assert_eq!(
        check_if_prerequisite(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![3, 0]]
        ),
        vec![true, false, true, false]
    );
}
