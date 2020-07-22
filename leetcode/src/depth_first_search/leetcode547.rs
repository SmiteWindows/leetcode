// https://leetcode.com/problems/friend-circles/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    let mut m = m;
    let mut res = 0;
    let n = m.len();
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            dfs(&mut m, &mut visited, i, n);
            res += 1;
        }
    }
    res
}

fn dfs(m: &mut Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize, n: usize) {
    for j in 0..n {
        if m[i][j] == 1 && !visited[j] {
            visited[j] = true;
            dfs(m, visited, j, n);
        }
    }
}
// depth_first_search union_find
#[test]
fn test2_547() {
    assert_eq!(
        find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1],]),
        1
    );
}
