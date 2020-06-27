// https://leetcode.com/problems/the-kth-factor-of-n/
pub fn kth_factor(n: i32, k: i32) -> i32 {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_1492() {
    assert_eq!(kth_factor(12, 3), 3);
    assert_eq!(kth_factor(7, 2), 7);
    assert_eq!(kth_factor(4, 4), -1);
    assert_eq!(kth_factor(1, 1), 1);
    assert_eq!(kth_factor(1000, 3), 4);
}
