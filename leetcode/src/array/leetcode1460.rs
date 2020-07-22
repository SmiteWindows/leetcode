// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-sub-arrays/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut target = target;
    let mut arr = arr;
    target.sort();
    arr.sort();
    target == arr
}
// array
#[test]
fn test1_1460() {
    assert_eq!(can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
    assert_eq!(can_be_equal(vec![7], vec![7]), true);
    assert_eq!(can_be_equal(vec![1, 12], vec![12, 1]), true);
    assert_eq!(can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
    assert_eq!(can_be_equal(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]), true);
}
