// https://leetcode.com/problems/fraction-to-recurring-decimal/
pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    todo!()
}
// math hash_table
#[test]
#[ignore]
fn test2_166() {
    assert_eq!(fraction_to_decimal(1, 2), String::from("0.5"));
    assert_eq!(fraction_to_decimal(2, 1), String::from("2"));
    assert_eq!(fraction_to_decimal(2, 3), String::from("0.(6)"));
}
