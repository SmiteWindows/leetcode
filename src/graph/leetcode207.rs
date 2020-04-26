// https://leetcode.com/problems/course-schedule/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut n = num_courses as usize;
    let mut indegrees = vec![0; n];
    let mut edges = vec![vec![]; n];
    let mut queue = vec![];
    for e in prerequisites {
        let u = e[1] as usize;
        let v = e[0] as usize;
        indegrees[v] += 1;
        edges[u].push(v);
    }
    for (u, &indegree) in indegrees.iter().enumerate().take(n) {
        if indegree == 0 {
            queue.push(u);
        }
    }
    while let Some(u) = queue.pop() {
        n -= 1;
        while let Some(v) = edges[u].pop() {
            indegrees[v] -= 1;
            if indegrees[v] == 0 {
                queue.push(v);
            }
        }
    }
    n == 0
}
// depth_first_search topological_sort graph breadth_first_search
#[test]
fn test1_207() {
    assert_eq!(can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
}
