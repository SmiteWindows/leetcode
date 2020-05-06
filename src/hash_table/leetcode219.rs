// https://leetcode.com/problems/contains-duplicate-ii/
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test1_219() {
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}
