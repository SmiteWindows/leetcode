// https://leetcode.com/problems/integer-to-roman/
pub fn int_to_roman(num: i32) -> String {
    todo!()
}
// math string
#[test]
#[ignore]
fn test2_12() {
    assert_eq!(int_to_roman(3), String::from("III"));
    assert_eq!(int_to_roman(4), String::from("IV"));
    assert_eq!(int_to_roman(9), String::from("IX"));
    assert_eq!(int_to_roman(58), String::from("LVIII"));
    assert_eq!(int_to_roman(1994), String::from("MCMXCIV"));
}
