// https://leetcode-cn.com/problems/minimum-height-trees/
// Runtime: 8 ms
// Memory Usage: 2.9 MB
use std::collections::VecDeque;
pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    if n == 1 {
        return vec![0];
    }
    let mut graph = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut degree = vec![0; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
        degree[u] += 1;
        degree[v] += 1;
    }

    let mut leaves = VecDeque::new();
    for (i, gi) in graph.iter().enumerate().take(n) {
        if gi.len() == 1 {
            leaves.push_back(i);
        }
    }
    let mut m = n;
    while m > 2 {
        m -= leaves.len();
        for _ in 0..leaves.len() {
            let u = leaves.pop_front().unwrap();
            visited[u] = true;
            for &v in &graph[u] {
                if !visited[v] {
                    degree[v] -= 1;
                    if degree[v] == 1 {
                        leaves.push_back(v);
                    }
                }
            }
        }
    }
    leaves.into_iter().map(|x| x as i32).collect()
}
// graph breadth_first_search
#[test]
fn test1_310() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_min_height_trees(4, vec2![[1, 0], [1, 2], [1, 3]]),
        vec![1]
    );
    assert_eq!(
        find_min_height_trees(6, vec2![[0, 3], [1, 3], [2, 3], [4, 3], [5, 4]]),
        vec![3, 4]
    );
}
