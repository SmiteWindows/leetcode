// https://leetcode.com/problems/valid-boomerang/
pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_1037() {
    assert_eq!(is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]), true);
    assert_eq!(
        is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
        false
    );
}
