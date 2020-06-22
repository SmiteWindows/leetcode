// https://leetcode.com/problems/smallest-integer-divisible-by-k/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    let mut x = 0;
    for i in 0..k {
        x *= 10;
        x += 1;
        x %= k;
        if x % k == 0 {
            return i + 1;
        }
    }
    -1
}
// math
#[test]
fn test1_1015() {
    assert_eq!(smallest_repunit_div_by_k(1), 1);
    assert_eq!(smallest_repunit_div_by_k(2), -1);
    assert_eq!(smallest_repunit_div_by_k(3), 3);
}
