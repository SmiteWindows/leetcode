// https://leetcode-cn.com/problems/powx-n/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn my_pow(mut x: f64, n: i32) -> f64 {
    let mut n = n as i64;

    if n < 0 {
        x = 1_f64 / x;
        n = -n;
    }
    fast_pow(x, n)
}

fn fast_pow(x: f64, n: i64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let half = fast_pow(x, n / 2);
    if n % 2 == 0 {
        half * half
    } else {
        half * half * x
    }
}
// math binary_search
#[test]
fn test1_50() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(my_pow(2.00000, 10), 1024.00000);
    assert_approx_eq!(my_pow(2.10000, 3), 9.261000000000001);
    assert_approx_eq!(my_pow(2.00000, -2), 0.25000);
}
