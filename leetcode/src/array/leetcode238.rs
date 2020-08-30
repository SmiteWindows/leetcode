// https://leetcode-cn.com/problems/product-of-array-except-self/
// Runtime: 4 ms
// Memory Usage: 3.1 MB
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![1; n];
    let mut product = 1;
    for i in 0..n {
        res[i] *= product;
        product *= nums[i];
    }
    product = 1;
    for i in (0..n).rev() {
        res[i] *= product;
        product *= nums[i];
    }
    res
}
// array
#[test]
fn test1_238() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}
