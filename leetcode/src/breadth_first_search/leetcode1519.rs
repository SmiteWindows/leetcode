// https://leetcode-cn.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/
// Runtime: 84 ms
// Memory Usage: 22.6 MB
pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut counts: Vec<usize> = vec![0; 26];
    let mut res = vec![0; n];
    let labels = labels.bytes().collect::<Vec<u8>>();
    dfs(0, &mut visited, &mut counts, &mut res, &adj, &labels);
    res
}

fn dfs(
    u: usize,
    visited: &mut Vec<bool>,
    counts: &mut Vec<usize>,
    sizes: &mut Vec<i32>,
    adj: &[Vec<usize>],
    labels: &[u8],
) {
    visited[u] = true;
    let i = (labels[u] - b'a') as usize;
    let last_count = counts[i];
    counts[i] += 1;
    for &v in adj[u].iter() {
        if !visited[v] {
            dfs(v, visited, counts, sizes, adj, labels);
        }
    }
    sizes[u] = (counts[i] - last_count) as i32;
}
// breadth_first_search depth_first_search
#[test]
fn test1_1519() {
    use leetcode_prelude::vec2;
    assert_eq!(
        count_sub_trees(
            7,
            vec2![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
            "abaedcd".to_string()
        ),
        vec![2, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        count_sub_trees(4, vec2![[0, 1], [1, 2], [0, 3]], "bbbb".to_string()),
        vec![4, 2, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            5,
            vec2![[0, 1], [0, 2], [1, 3], [0, 4]],
            "aabab".to_string()
        ),
        vec![3, 2, 1, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            6,
            vec2![[0, 1], [0, 2], [1, 3], [3, 4], [4, 5]],
            "cbabaa".to_string()
        ),
        vec![1, 2, 1, 1, 2, 1]
    );
    assert_eq!(
        count_sub_trees(
            7,
            vec2![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]],
            "aaabaaa".to_string()
        ),
        vec![6, 5, 4, 1, 3, 2, 1]
    );
}
