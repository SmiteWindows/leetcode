// https://leetcode-cn.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut n = n;
    let mut product = 1;
    let mut sum = 0;
    while n != 0 {
        let d = n % 10;
        product *= d;
        sum += d;
        n /= 10;
    }
    product - sum
}
// math
#[test]
fn test1_1281() {
    assert_eq!(subtract_product_and_sum(234), 15);
    assert_eq!(subtract_product_and_sum(4421), 21);
}
