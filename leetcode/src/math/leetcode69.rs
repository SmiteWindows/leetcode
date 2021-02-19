// https://leetcode-cn.com/problems/sqrtx/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    let x = x as f64;
    let mut x0 = x;
    let mut x1 = (x0 + x / x0) / 2.0;
    while (x0 - x1).abs() >= 1.0 {
        x0 = x1;
        x1 = (x0 + x / x0) / 2.0;
    }
    x1 as i32
}
// math binary_search
#[test]
fn test2_69() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
}
