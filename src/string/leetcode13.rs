// https://leetcode.com/problems/roman-to-integer/
pub fn roman_to_int(s: String) -> i32 {
    todo!()
}
// math string
#[test]
#[ignore]
fn test1_13() {
    assert_eq!(roman_to_int(String::from("III")), 3);
    assert_eq!(roman_to_int(String::from("IV")), 4);
    assert_eq!(roman_to_int(String::from("IX")), 9);
    assert_eq!(roman_to_int(String::from("LVIII")), 58);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
}
