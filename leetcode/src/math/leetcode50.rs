// https://leetcode-cn.com/problems/powx-n/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn my_pow(mut x: f64, n: i32) -> f64 {
    let mut n = n as i64;
    if n < 0 {
        x = 1_f64 / x;
        n = -n;
    }
    let mut res = 1_f64;
    let mut current_product = x;
    let mut i = n;
    while i > 0 {
        if i % 2 == 1 {
            res *= current_product;
        }
        current_product = current_product * current_product;
        i /= 2;
    }
    res
    //   x.powi(n)
}
// math binary_search
#[test]
fn test2_50() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(my_pow(2.00000, 10), 1024.00000);
    assert_approx_eq!(my_pow(2.10000, 3), 9.261000000000001);
    assert_approx_eq!(my_pow(2.00000, -2), 0.25000);
}
