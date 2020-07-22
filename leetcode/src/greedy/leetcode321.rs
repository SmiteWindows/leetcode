// https://leetcode.com/problems/create-maximum-number/
pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}
// greedy dynamic_programming
#[test]
#[ignore]
fn test1_321() {
    assert_eq!(
        max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
        vec![9, 8, 6, 5, 3]
    );
    assert_eq!(
        max_number(vec![6, 7], vec![6, 0, 4], 5),
        vec![6, 7, 6, 0, 4]
    );
    assert_eq!(max_number(vec![3, 9], vec![8, 9], 3), vec![9, 8, 9]);
}
