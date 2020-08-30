// https://leetcode-cn.com/problems/all-paths-from-source-to-target/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path = vec![];
    let n = graph.len();
    dfs(0, &mut path, &mut res, &graph, n);
    res
}

fn dfs(u: i32, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>, graph: &[Vec<i32>], n: usize) {
    path.push(u);
    if u as usize == n - 1 {
        paths.push(path.clone());
    } else {
        for &v in &graph[u as usize] {
            dfs(v, path, paths, graph, n);
        }
    }
    path.pop();
}
#[test]
fn test797() {
    assert_eq!(
        all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        vec![vec![0, 1, 3], vec![0, 2, 3]]
    );
}
