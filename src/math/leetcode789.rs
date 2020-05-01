// https://leetcode.com/problems/escape-the-ghosts/
pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_789() {
    assert_eq!(
        escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1]),
        true
    );
    assert_eq!(escape_ghosts(vec![vec![1, 0]], vec![2, 0]), false);
    assert_eq!(escape_ghosts(vec![vec![2, 0]], vec![1, 0]), false);
}
