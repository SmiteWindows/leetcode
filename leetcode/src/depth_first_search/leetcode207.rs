// https://leetcode-cn.com/problems/course-schedule/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = num_courses as usize;
    let mut edges = vec![vec![]; n];
    let mut flags = vec![0; n];
    for e in prerequisites {
        let u = e[1] as usize;
        let v = e[0] as usize;
        edges[u].push(v);
    }
    for u in 0..n {
        if !dfs(&edges, &mut flags, u) {
            return false;
        }
    }
    true
}

fn dfs(edges: &[Vec<usize>], flags: &mut Vec<i32>, i: usize) -> bool {
    if flags[i] == 1 {
        return false;
    }
    if flags[i] == -1 {
        return true;
    }
    flags[i] = 1;
    for &j in &edges[i] {
        if !dfs(edges, flags, j) {
            return false;
        }
    }
    flags[i] = -1;
    true
}
// depth_first_search topological_sort graph breadth_first_search
#[test]
fn test3_207() {
    assert_eq!(can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
}
