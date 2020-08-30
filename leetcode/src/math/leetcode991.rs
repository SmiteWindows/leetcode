// https://leetcode-cn.com/problems/broken-calculator/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn broken_calc(x: i32, y: i32) -> i32 {
    let mut y = y;
    let mut res = 0;
    while y > x {
        if y % 2 == 0 {
            y /= 2;
        } else {
            y += 1;
        }
        res += 1;
    }
    res + x - y
}
// math greedy
#[test]
fn test1_991() {
    assert_eq!(broken_calc(2, 3), 2);
    assert_eq!(broken_calc(5, 8), 2);
    assert_eq!(broken_calc(3, 10), 3);
    assert_eq!(broken_calc(1024, 1), 1023);
}
