// https://leetcode-cn.com/problems/convert-a-number-to-hexadecimal/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn to_hex(num: i32) -> String {
    format!("{:x}", num)
}
// bit_manipulation
#[test]
fn test1_405() {
    assert_eq!(to_hex(26), "1a".to_string());
    assert_eq!(to_hex(-1), "ffffffff".to_string());
}
