// https://leetcode-cn.com/problems/check-if-word-is-valid-after-substitutions/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
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
    assert_eq!(is_valid(String::from("aabcbc")), true);
    assert_eq!(is_valid(String::from("abcabcababcc")), true);
    assert_eq!(is_valid(String::from("abccba")), false);
    assert_eq!(is_valid(String::from("cababc")), false);
}
