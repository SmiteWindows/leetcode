// https://leetcode.com/problems/remove-outermost-parentheses/
pub fn remove_outer_parentheses(s: String) -> String {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_1021() {
    assert_eq!(
        remove_outer_parentheses(String::from("(()())(())")),
        String::from("()()()")
    );
    assert_eq!(
        remove_outer_parentheses(String::from("(()())(())(()(()))")),
        String::from("()()()()(())")
    );
    assert_eq!(
        remove_outer_parentheses(String::from("()()")),
        String::from("")
    );
}
