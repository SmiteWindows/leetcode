// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_1317() {
    assert_eq!(get_no_zero_integers(2), vec![1, 1]);
    assert_eq!(get_no_zero_integers(11), vec![2, 9]);
    assert_eq!(get_no_zero_integers(10000), vec![1, 9999]);
    assert_eq!(get_no_zero_integers(69), vec![1, 68]);
    assert_eq!(get_no_zero_integers(1010), vec![11, 999]);
}
