// https://leetcode.com/problems/remove-invalid-parentheses/
pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    todo!()
}
// depth_first_search breadth_first_search
#[test]
#[ignore]
fn test1_301() {
    assert_eq!(
        remove_invalid_parentheses(String::from("()())()")),
        vec![String::from("()()()"), String::from("(())()")]
    );
    assert_eq!(
        remove_invalid_parentheses(String::from("(a)())()")),
        vec![String::from("(a)()()"), String::from("(a())()")]
    );
    assert_eq!(
        remove_invalid_parentheses(String::from(")(")),
        vec![String::from("")]
    );
}
