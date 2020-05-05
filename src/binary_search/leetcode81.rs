// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test1_81() {
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}
