// https://leetcode.com/problems/valid-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => match stack.pop() {
                Some(t) => {
                    if !((t == '{' && c == '}') || (t == '(' && c == ')') || (t == '[' && c == ']'))
                    {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            },
            _ => {}
        }
    }
    stack.is_empty()
}
// stack string
#[test]
fn test2_20() {
    assert_eq!(is_valid(String::from("()")), true);
    assert_eq!(is_valid(String::from("()[]{}")), true);
    assert_eq!(is_valid(String::from("(]")), false);
    assert_eq!(is_valid(String::from("([)]")), false);
    assert_eq!(is_valid(String::from("{[]}")), true);
}
