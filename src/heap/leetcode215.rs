// https://leetcode.com/problems/kth-largest-element-in-an-array/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// divide_and_conquer heap
#[test]
#[ignore]
fn test1_215() {
    assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
