// https://leetcode.com/problems/maximum-product-subarray/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::mem::swap;
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut min = res;
    let mut max = res;
    let n = nums.len();
    for i in 1..n {
        let x = nums[i];
        if x < 0 {
            swap(&mut min, &mut max);
        }
        max = x.max(max * x);
        min = x.min(min * x);
        res = res.max(max);
    }
    res
}
// dynamic_programming array
#[test]
fn test2_152() {
    assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(max_product(vec![-2, 0, -1]), 0);
}
