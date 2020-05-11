// https://leetcode.com/problems/to-lower-case/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn to_lower_case(str: String) -> String {
    str.chars()
    .map(|c| {
        if c as u8 >= b'A' && c as u8 <= b'Z' {
            (c as u8 + (b'a' - b'A')) as char
        } else {
            c
        }
    })
    .collect()
}
// string
#[test]
fn test1_709() {
    assert_eq!(to_lower_case(String::from("Hello")), String::from("hello"));
    assert_eq!(to_lower_case(String::from("here")), String::from("here"));
    assert_eq!(
        to_lower_case(String::from("LOVELY")),
        String::from("lovely")
    );
}
