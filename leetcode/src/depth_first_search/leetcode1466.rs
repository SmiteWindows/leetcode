// https://leetcode-cn.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
// Runtime: 28 ms
// Memory Usage: 6.8 MB
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    for connection in connections {
        let u = connection[0] as usize;
        let v = connection[1] as usize;
        graph[u].push((v, true));
        graph[v].push((u, false));
    }
    let mut res = 0;
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, &mut visited, &mut res, &graph);
    res
}

fn dfs(u: usize, visited: &mut [bool], changed: &mut i32, graph: &[Vec<(usize, bool)>]) {
    for &(v, d) in &graph[u] {
        if !visited[v] {
            if d {
                *changed += 1;
            }
            visited[v] = true;
            dfs(v, visited, changed, graph);
        }
    }
}
// tree depth_first_search
#[test]
fn test2_1466() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_reorder(6, vec2![[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]),
        3
    );
    assert_eq!(min_reorder(5, vec2![[1, 0], [1, 2], [3, 2], [3, 4]]), 2);
    assert_eq!(min_reorder(3, vec2![[1, 0], [2, 0]]), 0);
}
