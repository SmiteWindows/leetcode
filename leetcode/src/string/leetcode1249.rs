// https://leetcode-cn.com/problems/minimum-remove-to-make-valid-parentheses/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
use std::collections::HashSet;
pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack = vec![];
    let mut res = "".to_string();
    let mut remove = HashSet::new();
    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                stack.push(i);
            }
            ')' => {
                if stack.pop().is_none() {
                    remove.insert(i);
                }
            }
            _ => {}
        }
    }
    for i in stack {
        remove.insert(i);
    }
    for (i, c) in s.char_indices() {
        if !remove.contains(&i) {
            res.push(c);
        }
    }
    res
}
// stack string
#[test]
fn test2_1249() {
    assert_eq!(
        min_remove_to_make_valid("lee(t(c)o)de)")),
        "lee(t(c)o)de")
    );
    assert_eq!(
        min_remove_to_make_valid("a)b(c)d")),
        "ab(c)d")
    );
    assert_eq!(
        min_remove_to_make_valid("))((")),
        "")
    );
    assert_eq!(
        min_remove_to_make_valid("(a(b(c)d)")),
        "a(b(c)d)")
    );
}
