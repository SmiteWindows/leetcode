// https://leetcode-cn.com/problems/check-if-word-is-valid-after-substitutions/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        let n = stack.len();
        if n > 1 && c == 'c' && stack[n - 1] == 'b' && stack[n - 2] == 'a' {
            stack.pop();
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.is_empty()
}
// stack string
#[test]
fn test1_1003() {
    assert_eq!(is_valid("aabcbc".to_string()), true);
    assert_eq!(is_valid("abcabcababcc".to_string()), true);
    assert_eq!(is_valid("abccba".to_string()), false);
    assert_eq!(is_valid("cababc".to_string()), false);
}
