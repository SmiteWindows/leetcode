// https://leetcode-cn.com/problems/1-bit-and-2-bit-characters/
pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    bits.into_iter()
        .rev()
        .skip(1)
        .take_while(|&x| x == 1)
        .count()
        % 2
        == 0
}
// Runtime: 0 ms
// Memory Usage: 2.1 MB
// ✔
// array
#[test]
fn test1_717() {
    assert_eq!(is_one_bit_character(vec![1, 0, 0]), true);
    assert_eq!(is_one_bit_character(vec![1, 1, 1, 0]), false);
}
