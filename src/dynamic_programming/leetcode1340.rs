// https://leetcode.com/problems/jump-game-v/
pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1340() {
    assert_eq!(max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
    assert_eq!(max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
    assert_eq!(max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    assert_eq!(max_jumps(vec![7, 1, 7, 1, 7, 1], 2), 2);
    assert_eq!(max_jumps(vec![66], 1), 1);
}
