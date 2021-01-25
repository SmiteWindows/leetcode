// https://leetcode-cn.com/problems/airplane-seat-assignment-probability/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
    if n > 1 {
        0.5
    } else {
        1.0
    }
}
// math dynamic_programming brainteaser
#[test]
fn test1_1227() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(nth_person_gets_nth_seat(1), 1.00000);
    assert_approx_eq!(nth_person_gets_nth_seat(2), 0.50000);
}
