// https://leetcode-cn.com/problems/loud-and-rich/
// Runtime: 24 ms
// Memory Usage: 3.8 MB
use std::collections::HashSet;
pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let n = quiet.len();
    let mut graph = vec![HashSet::new(); n];
    for e in richer {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[v].insert(u);
    }
    let mut res = vec![n; n];
    for i in 0..n {
        dfs(i, &mut res, &graph, &quiet, n);
    }
    res.into_iter().map(|x| x as i32).collect()
}

fn dfs(
    u: usize,
    stack: &mut Vec<usize>,
    graph: &[HashSet<usize>],
    quiet: &[i32],
    n: usize,
) -> usize {
    if stack[u] == n {
        stack[u] = u;
        for &v in &graph[u] {
            let w = dfs(v, stack, graph, quiet, n);
            if quiet[w] < quiet[stack[u]] {
                stack[u] = w;
            }
        }
    }
    stack[u]
}
// depth_first_search
#[test]
fn test1_851() {
    use leetcode_prelude::vec2;
    assert_eq!(
        loud_and_rich(
            vec2![[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]],
            vec![3, 2, 5, 4, 6, 1, 7, 0]
        ),
        vec![5, 5, 2, 5, 4, 5, 6, 7]
    );
}
