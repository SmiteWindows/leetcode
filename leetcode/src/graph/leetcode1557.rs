// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/
// Runtime: 52 ms
// Memory Usage: 8.7 MB
// ✔
use std::collections::HashSet;
pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let dependent: HashSet<_> = edges.into_iter().map(|x| x[1]).collect();
    let result: HashSet<_> = (0..n).collect();
    result.difference(&dependent).cloned().collect()
}
// graph
#[test]
fn test1_1557() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(
        find_smallest_set_of_vertices(6, vec2![[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]]),
        vec![0, 3]
    );
    assert_eq_sorted!(
        find_smallest_set_of_vertices(5, vec2![[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]),
        vec![0, 2, 3]
    );
}

pub fn _find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut indegree = vec![0; n];
    for e in edges {
        let to = e[1] as usize;
        indegree[to] += 1;
    }
    let mut res = vec![];
    for (i, &a) in indegree.iter().enumerate().take(n) {
        if a == 0 {
            res.push(i as i32);
        }
    }
    res
}
