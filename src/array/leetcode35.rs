// https://leetcode.com/problems/search-insert-position/
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test2_35() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
}
