// https://leetcode-cn.com/problems/maximum-product-of-three-numbers/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;
    let mut max3 = i32::MIN;
    for &n in &nums {
        if n <= min1 {
            min2 = min1;
            min1 = n;
        } else if n <= min2 {
            min2 = n;
        }
        if n >= max1 {
            max3 = max2;
            max2 = max1;
            max1 = n;
        } else if n >= max2 {
            max3 = max2;
            max2 = n;
        } else if n >= max3 {
            max3 = n;
        }
    }
    (min1 * min2 * max1).max(max1 * max2 * max3)
}
// math array
#[test]
fn test1_628() {
    assert_eq!(maximum_product(vec![1, 2, 3]), 6);
    assert_eq!(maximum_product(vec![1, 2, 3, 4]), 24);
}
