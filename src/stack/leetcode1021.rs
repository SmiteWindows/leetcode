// https://leetcode.com/problems/remove-outermost-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn remove_outer_parentheses(s: String) -> String {
    let mut res = String::from("");
    let mut count = 0;
    for c in s.chars() {
        if c == '(' {
            count += 1;
            if count > 1 {
                res.push(c);
            }
        } else {
            count -= 1;
            if count != 0 {
                res.push(c);
            }
        }
    }
    res
}
// stack
#[test]
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
