// https://leetcode.com/problems/longest-turbulent-subarray/
pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
    todo!()
}
// sliding_window array dynamic_programming
#[test]
#[ignore]
fn test3_978() {
    assert_eq!(max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    assert_eq!(max_turbulence_size(vec![4, 8, 12, 16]), 2);
    assert_eq!(max_turbulence_size(vec![100]), 1);
}
