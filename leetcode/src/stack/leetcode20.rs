// https://leetcode-cn.com/problems/valid-parentheses/
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
fn test1_20() {
    assert_eq!(is_valid("()".to_string()), true);
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("(]".to_string()), false);
    assert_eq!(is_valid("([)]".to_string()), false);
    assert_eq!(is_valid("{[]}".to_string()), true);
}
