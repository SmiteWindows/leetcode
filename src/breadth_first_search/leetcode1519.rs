// https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/
// Runtime: 116 ms
// Memory Usage: 43 MB
pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut res = vec![0; n];
    let labels = labels.bytes().collect::<Vec<u8>>();
    visited[0] = true;
    dfs(0, &mut visited, &mut res, &adj, &labels);
    res
}

fn dfs(
    u: usize,
    visited: &mut Vec<bool>,
    sizes: &mut Vec<i32>,
    adj: &[Vec<usize>],
    labels: &[u8],
) -> [i32; 26] {
    let mut count = [0; 26];
    count[(labels[u] - b'a') as usize] = 1;
    for &v in adj[u].iter() {
        if !visited[v] {
            visited[v] = true;
            let subtree = dfs(v, visited, sizes, adj, labels);
            for i in 0..26 {
                count[i] += subtree[i];
            }
        }
    }
    sizes[u] = count[(labels[u] - b'a') as usize];
    count
}
// breadth_first_search depth_first_search
#[test]
fn test1_1519() {
    assert_eq!(
        count_sub_trees(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            "abaedcd".to_string()
        ),
        vec![2, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            4,
            vec![vec![0, 1], vec![1, 2], vec![0, 3]],
            "bbbb".to_string()
        ),
        vec![4, 2, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            5,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]],
            "aabab".to_string()
        ),
        vec![3, 2, 1, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![3, 4], vec![4, 5]],
            "cbabaa".to_string()
        ),
        vec![1, 2, 1, 1, 2, 1]
    );
    assert_eq!(
        count_sub_trees(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6]
            ],
            "aaabaaa".to_string()
        ),
        vec![6, 5, 4, 1, 3, 2, 1]
    );
}
