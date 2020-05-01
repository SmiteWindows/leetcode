// https://leetcode.com/problems/continuous-subarray-sum/
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    todo!()
}
// math dynamic_programming
#[test]
fn test2_523() {
    assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
}
