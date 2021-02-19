// https://leetcode-cn.com/problems/escape-the-ghosts/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let min = dist(&target, &[0, 0]);
    for ghost in &ghosts {
        if dist(ghost, &target) <= min {
            return false;
        }
    }
    true
}

fn dist(a: &[i32], b: &[i32]) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}
// math
#[test]
fn test1_789() {
    use leetcode_prelude::vec2;
    assert_eq!(escape_ghosts(vec2![[1, 0], [0, 3]], vec![0, 1]), true);
    assert_eq!(escape_ghosts(vec2![[1, 0]], vec![2, 0]), false);
    assert_eq!(escape_ghosts(vec2![[2, 0]], vec![1, 0]), false);
}
