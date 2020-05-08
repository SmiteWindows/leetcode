// https://leetcode.com/problems/number-complement/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_complement(num: i32) -> i32 {
    let mut mask = !0;
    while mask & num != 0 {
        mask <<= 1;
    }
    !mask & !num
}
// bit_manipulation
#[test]
fn test1_476() {
    assert_eq!(find_complement(5), 2);
    assert_eq!(find_complement(1), 0);
}
