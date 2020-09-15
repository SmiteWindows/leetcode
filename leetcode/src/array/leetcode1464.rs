// https://leetcode-cn.com/problems/maximum-product-of-two-elements-in-an-array/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn max_product(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    (nums[n - 1] - 1) * (nums[n - 2] - 1)
}
// array
#[test]
fn test1_1464() {
    assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
    assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(max_product(vec![3, 7]), 12);
}
