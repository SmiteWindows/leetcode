// https://leetcode-cn.com/problems/the-kth-factor-of-n/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn kth_factor(n: i32, mut k: i32) -> i32 {
    for i in 1..=n {
        if n % i == 0 {
            k -= 1;
            if k == 0 {
                return i;
            }
        }
    }
    -1
}
// math
#[test]
fn test1_1492() {
    assert_eq!(kth_factor(12, 3), 3);
    assert_eq!(kth_factor(7, 2), 7);
    assert_eq!(kth_factor(4, 4), -1);
    assert_eq!(kth_factor(1, 1), 1);
    assert_eq!(kth_factor(1000, 3), 4);
}
