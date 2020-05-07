// https://leetcode.com/problems/minimum-absolute-difference/
pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1200() {
    assert_eq!(
        minimum_abs_difference(vec![4, 2, 1, 3]),
        vec![vec![1, 2], vec![2, 3], vec![3, 4]]
    );
    assert_eq!(
        minimum_abs_difference(vec![1, 3, 6, 10, 15]),
        vec![vec![1, 3]]
    );
    assert_eq!(
        minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
        vec![vec![-14, -10], vec![19, 23], vec![23, 27]]
    );
}
