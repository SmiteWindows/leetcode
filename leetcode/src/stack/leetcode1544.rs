// https://leetcode-cn.com/problems/make-the-string-great/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn make_good(s: String) -> String {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if let Some(&last) = stack.last() {
            if c.is_uppercase() && last.is_lowercase() && c.to_ascii_lowercase() == last {
                stack.pop();
                continue;
            }
            if c.is_lowercase() && last.is_uppercase() && c.to_ascii_uppercase() == last {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
    }
    stack.into_iter().collect()
}
// string stack
#[test]
fn test2_1544() {
    assert_eq!(make_good("leEeetcode".to_string()), "leetcode".to_string());
    assert_eq!(make_good("abBAcC".to_string()), "".to_string());
    assert_eq!(make_good("s".to_string()), "s".to_string());
}
