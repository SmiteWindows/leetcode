// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1431() {
    assert_eq!(k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
    assert_eq!(k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
    assert_eq!(k_length_apart(vec![1, 1, 1, 1, 1], 0), true);
    assert_eq!(k_length_apart(vec![0, 1, 0, 1], 1), true);
}
