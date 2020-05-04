// https://leetcode.com/problems/backspace-string-compare/
pub fn backspace_compare(s: String, t: String) -> bool {
    todo!()
}
// two_pointers stack
#[test]
#[ignore]
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
