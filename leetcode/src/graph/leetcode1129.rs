// https://leetcode-cn.com/problems/shortest-path-with-alternating-colors/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::VecDeque;
pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    let n = n as usize;
    let mut graph = vec![vec![Vec::new(); n]; 2];
    let mut visited = vec![vec![false; n]; 2];
    let mut res = vec![-1; n];
    for e in red_edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[0][u].push(v);
    }
    for e in blue_edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[1][u].push(v);
    }
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    queue.push_back((0, 1, 0));
    visited[0][0] = true;
    visited[1][0] = true;
    while let Some((u, c, d)) = queue.pop_front() {
        if res[u] == -1 {
            res[u] = d as i32;
        }
        for &v in &graph[c][u] {
            if !visited[1 - c][v] {
                visited[1 - c][v] = true;
                queue.push_back((v, 1 - c, d + 1));
            }
        }
    }
    res
}
// graph breadth_first_search
#[test]
fn test1_1129() {
    use leetcode_prelude::vec2;
    assert_eq!(
        shortest_alternating_paths(3, vec2![[0, 1], [1, 2]], vec![]),
        vec![0, 1, -1]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec2![[0, 1]], vec2![[2, 1]]),
        vec![0, 1, -1]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec2![[1, 0]], vec2![[2, 1]]),
        vec![0, -1, -1]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec2![[0, 1]], vec2![[1, 2]]),
        vec![0, 1, 2]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec2![[0, 1], [0, 2]], vec2![[1, 0]]),
        vec![0, 1, 1]
    );
}
