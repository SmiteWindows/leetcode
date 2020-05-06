// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
    todo!()
}
// greedy array
#[test]
#[ignore]
fn test2_1007() {
    assert_eq!(
        min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
        2
    );
    assert_eq!(
        min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
        2
    );
}
