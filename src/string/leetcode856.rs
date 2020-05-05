// https://leetcode.com/problems/score-of-parentheses/
pub fn score_of_parentheses(s: String) -> i32 {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test2_856() {
    assert_eq!(score_of_parentheses(String::from("()")), 1);
    assert_eq!(score_of_parentheses(String::from("(())")), 2);
    assert_eq!(score_of_parentheses(String::from("()()")), 2);
    assert_eq!(score_of_parentheses(String::from("(()(()))")), 6);
}
