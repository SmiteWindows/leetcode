// https://leetcode.com/problems/valid-parentheses/
pub fn is_valid(s: String) -> bool {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test1_20() {
    assert_eq!(is_valid(String::from("()")), true);
    assert_eq!(is_valid(String::from("()[]{}")), true);
    assert_eq!(is_valid(String::from("(]")), false);
    assert_eq!(is_valid(String::from("([)]")), false);
    assert_eq!(is_valid(String::from("{[]}")), true);
}
