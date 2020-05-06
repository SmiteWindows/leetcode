// https://leetcode.com/problems/longest-valid-parentheses/
pub fn longest_valid_parentheses(s: String) -> i32 {
    todo!()
}
// string dynamic_programming
#[test]
#[ignore]
fn test1_32() {
    assert_eq!(longest_valid_parentheses(String::from("(()")), 2);
    assert_eq!(longest_valid_parentheses(String::from(")()())")), 4);
}
