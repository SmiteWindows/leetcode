// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test2_34() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
}
