// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/
pub fn sum_zero(n: i32) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1304() {
    assert_eq!(sum_zero(5), vec![-7, -1, 1, 3, 4]);
    // assert_eq!(sum_zero(5), vec![-5, -1, 1, 2, 3]);
    // assert_eq!(sum_zero(5), vec![-3, -1, 2, -2, 4]);
    assert_eq!(sum_zero(3), vec![-1, 0, 1]);
    assert_eq!(sum_zero(1), vec![0]);
}
