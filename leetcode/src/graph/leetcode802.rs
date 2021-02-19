// https://leetcode-cn.com/problems/find-eventual-safe-states/
// Runtime: 32 ms
// Memory Usage: 2.6 MB
pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut res = Vec::new();
    let mut state = vec![0; n];
    for i in 0..n {
        if dfs(i, &mut state, &graph) {
            res.push(i as i32);
        }
    }
    res
}

fn dfs(u: usize, state: &mut [i32], graph: &[Vec<i32>]) -> bool {
    match state[u] {
        3 => false,
        2 => true,
        1 => {
            state[u] = 3;
            false
        }
        _ => {
            state[u] = 1;
            let mut s = 2;
            for &v in &graph[u] {
                if !dfs(v as usize, state, graph) {
                    s = 3;
                }
            }
            state[u] = s;
            state[u] == 2
        }
    }
}
// graph depth_first_search
#[test]
fn test1_802() {
    use leetcode_prelude::vec2;
    assert_eq!(
        eventual_safe_nodes(vec2![[1, 2], [2, 3], [5], [0], [5], [], []]),
        vec![2, 4, 5, 6]
    );
}
