// https://leetcode-cn.com/problems/minimum-add-to-make-parentheses-valid/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    for c in s.chars() {
        match c {
            '(' => {
                stack.push(c);
            }
            ')' => {
                if stack.is_empty() {
                    res += 1;
                } else {
                    stack.pop();
                }
            }
            _ => {}
        }
    }
    res + stack.len() as i32
}
// stack greedy
#[test]
fn test1_921() {
    assert_eq!(min_add_to_make_valid("())".to_string()), 1);
    assert_eq!(min_add_to_make_valid("(((".to_string()), 3);
    assert_eq!(min_add_to_make_valid("()".to_string()), 0);
    assert_eq!(min_add_to_make_valid("()))((".to_string()), 4);
}
