// https://leetcode.com/problems/uncrossed-lines/
pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1035() {
    assert_eq!(max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
    assert_eq!(
        max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
    assert_eq!(
        max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}
