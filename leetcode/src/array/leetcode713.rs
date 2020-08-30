// https://leetcode-cn.com/problems/subarray-product-less-than-k/
// Runtime: 12 ms
// Memory Usage: 2.3 MB
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut i = 0;
    let mut product = 1;
    let mut res = 0;
    for j in 0..n {
        product *= nums[j];
        while i <= j && product >= k {
            product /= nums[i];
            i += 1;
        }
        res += (j + 1 - i) as i32;
    }
    res
}
// array two_pointers
#[test]
fn test2_713() {
    assert_eq!(num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
}
