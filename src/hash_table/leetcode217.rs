// https://leetcode.com/problems/contains-duplicate/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test1_217() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}
