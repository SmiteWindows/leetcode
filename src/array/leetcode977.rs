// https://leetcode.com/problems/squares-of-a-sorted-array/
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    todo!()
}
// two_pointers array
#[test]
#[ignore]
fn test2_977() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}