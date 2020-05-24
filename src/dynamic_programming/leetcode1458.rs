// https://leetcode.com/problems/max-dot-product-of-two-subsequences/
pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1458() {
    assert_eq!(max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]), 18);
    assert_eq!(max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
    assert_eq!(max_dot_product(vec![-1, -1], vec![1, 1]), -1);
}
