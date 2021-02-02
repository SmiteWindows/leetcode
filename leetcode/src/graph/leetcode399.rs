// https://leetcode-cn.com/problems/evaluate-division/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::{BTreeSet, HashMap};
pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let m = equations.len();
    let mut symbols = BTreeSet::new();
    let mut ids = HashMap::new();
    for eq in &equations {
        symbols.insert(eq[0].clone());
        symbols.insert(eq[1].clone());
    }
    for (i, s) in symbols.into_iter().enumerate() {
        ids.insert(s, i);
    }
    let n = ids.len();
    let mut graph = vec![Vec::new(); n];

    for i in 0..m {
        let u = ids[&equations[i][0]];
        let v = ids[&equations[i][1]];
        graph[u].push((v, values[i]));
        graph[v].push((u, 1.0 / values[i]));
    }
    let mut res = Vec::new();
    for query in queries {
        if ids.contains_key(&query[0]) && ids.contains_key(&query[1]) {
            let u = ids[&query[0]];
            let v = ids[&query[1]];
            let mut product = -1.0;
            let mut visited = vec![false; n];
            let mut path = Vec::new();
            dfs(u, v, &mut visited, &mut path, &mut product, &graph);
            res.push(product);
        } else {
            res.push(-1.0);
        }
    }
    res
}

fn dfs(
    u: usize,
    v: usize,
    visited: &mut Vec<bool>,
    path: &mut Vec<f64>,
    product: &mut f64,
    graph: &[Vec<(usize, f64)>],
) {
    visited[u] = true;
    if u == v {
        *product = path.iter().fold(1.0, |a, v| a * v);
    } else {
        for e in &graph[u] {
            if !visited[e.0] {
                path.push(e.1);
                dfs(e.0, v, visited, path, product, graph);
                path.pop();
            }
        }
    }
    visited[u] = false;
}
// graph union_find
#[test]
fn test2_399() {
    use leetcode_prelude::vec2_string;
    assert_eq!(
        calc_equation(
            vec2_string![["a", "b"], ["b", "c"]],
            vec![2.0, 3.0],
            vec2_string![["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
        ),
        vec![6.0, 0.5, -1.0, 1.0, -1.0]
    );
}
