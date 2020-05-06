// https://leetcode.com/problems/unique-number-of-occurrences/
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1207() {
    assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(unique_occurrences(vec![1, 2]), false);
    assert_eq!(
        unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
        true
    );
}
