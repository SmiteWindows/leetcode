// https://leetcode.com/problems/valid-number/
pub fn is_number(s: String) -> bool {
    todo!()
}
// string math
#[test]
#[ignore]
fn test2_65() {
    assert_eq!(is_number(String::from("0")), true);
    assert_eq!(is_number(String::from(" 0.1 ")), true);
    assert_eq!(is_number(String::from("abc")), false);
    assert_eq!(is_number(String::from("1 a")), false);
    assert_eq!(is_number(String::from("2e10")), true);
    assert_eq!(is_number(String::from(" -90e3   ")), true);
    assert_eq!(is_number(String::from(" 1e")), false);
    assert_eq!(is_number(String::from("e3")), false);
    assert_eq!(is_number(String::from(" 6e-1")), true);
    assert_eq!(is_number(String::from(" 99e2.5 ")), false);
    assert_eq!(is_number(String::from("53.5e93")), true);
    assert_eq!(is_number(String::from(" --6 ")), false);
    assert_eq!(is_number(String::from("-+3")), false);
    assert_eq!(is_number(String::from("95a54e53")), false);
}
