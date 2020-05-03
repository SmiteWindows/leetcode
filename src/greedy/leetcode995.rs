// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/
pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// sliding_window greedy
#[test]
#[ignore]
fn test2_995() {
    assert_eq!(min_k_bit_flips(vec![0, 1, 0], 1), 2);
    assert_eq!(min_k_bit_flips(vec![1, 1, 0], 2), -1);
    assert_eq!(min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3), 3);
}
