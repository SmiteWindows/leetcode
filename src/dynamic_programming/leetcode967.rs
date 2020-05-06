// https://leetcode.com/problems/numbers-with-same-consecutive-differences/
pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_967() {
    assert_eq!(nums_same_consec_diff(3, 7), vec![181, 292, 707, 818, 929]);
    assert_eq!(
        nums_same_consec_diff(2, 1),
        vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
    );
}
