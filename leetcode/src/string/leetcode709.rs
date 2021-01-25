// https://leetcode-cn.com/problems/to-lower-case/
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
    assert_eq!(to_lower_case("Hello".to_string()), "hello".to_string());
    assert_eq!(to_lower_case("here".to_string()), "here".to_string());
    assert_eq!(to_lower_case("LOVELY".to_string()), "lovely".to_string());
}
