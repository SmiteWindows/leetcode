// https://leetcode-cn.com/problems/course-schedule-ii/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
use std::collections::VecDeque;
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let n = num_courses as usize;
    let mut edges = vec![vec![]; n];
    let mut indegrees = vec![0; n];
    let mut queue = VecDeque::new();
    for e in prerequisites {
        let u = e[1] as usize;
        let v = e[0] as usize;
        edges[u].push(v);
        indegrees[v] += 1;
    }
    for (u, &indegree) in indegrees.iter().enumerate().take(n) {
        if indegree == 0 {
            queue.push_back(u);
        }
    }
    while let Some(u) = queue.pop_front() {
        res.push(u);
        for &v in &edges[u] {
            indegrees[v] -= 1;
            if indegrees[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if res.len() == n {
        res.into_iter().map(|v| v as i32).collect()
    } else {
        vec![]
    }
}
// depth_first_search topological_sort graph breadth_first_search
#[test]
fn test1_210() {
    assert_eq!(find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
    // assert_eq!(
    //     find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
    //     vec![0, 2, 1, 3]
    // );
}
