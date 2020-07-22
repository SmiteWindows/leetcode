// https://leetcode.com/problems/powx-n/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut n = n as i64;
    let mut x = x;
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
    // if n == 0 {
    //     1.0
    // } else if n % 2 == 0 {
    //     my_pow(x * x, n / 2)
    // } else if n < 0 {
    //     1.0 / my_pow(x, -n)
    // } else {
    //     x * my_pow(x * x, n / 2)
    // }
}
// math binary_search
#[test]
fn test2_50() {
    assert_eq!(my_pow(2.00000, 10), 1024.00000);
    assert_eq!(my_pow(2.10000, 3), 9.261000000000001);
    assert_eq!(my_pow(2.00000, -2), 0.25000);
}
