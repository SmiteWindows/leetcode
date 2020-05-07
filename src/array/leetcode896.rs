// https://leetcode.com/problems/monotonic-array/
pub fn is_monotonic(a: Vec<i32>) -> bool {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_896() {
    assert_eq!(is_monotonic(vec![1, 2, 2, 3]), true);
    assert_eq!(is_monotonic(vec![6, 5, 4, 4]), true);
    assert_eq!(is_monotonic(vec![1, 3, 2]), false);
    assert_eq!(is_monotonic(vec![1, 2, 4, 5]), true);
    assert_eq!(is_monotonic(vec![1, 1, 1]), true);
}
