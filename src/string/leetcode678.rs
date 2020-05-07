// https://leetcode.com/problems/valid-parenthesis-string/
pub fn check_valid_string(s: String) -> bool {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_678() {
    assert_eq!(check_valid_string(String::from("()")), true);
    assert_eq!(check_valid_string(String::from("(*)")), false);
    assert_eq!(check_valid_string(String::from("(*))")), true);
}
