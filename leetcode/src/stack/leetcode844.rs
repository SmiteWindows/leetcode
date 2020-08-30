// https://leetcode-cn.com/problems/backspace-string-compare/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn backspace_compare(s: String, t: String) -> bool {
    get_backspaced(s) == get_backspaced(t)
}

fn get_backspaced(s: String) -> Vec<char> {
    let mut chars = Vec::with_capacity(s.len());
    for c in s.chars() {
        if c == '#' {
            chars.pop();
        } else {
            chars.push(c);
        }
    }
    chars
}
// two_pointers stack
#[test]
fn test2_844() {
    assert_eq!(
        backspace_compare(String::from("ab#c"), String::from("ad#c")),
        true
    );
    assert_eq!(
        backspace_compare(String::from("ab##"), String::from("c#d#")),
        true
    );
    assert_eq!(
        backspace_compare(String::from("a##c"), String::from("#a#c")),
        true
    );
    assert_eq!(
        backspace_compare(String::from("a#c"), String::from("b")),
        false
    );
}
