// https://leetcode.com/problems/to-lower-case/
pub fn to_lower_case(str: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_709() {
    assert_eq!(to_lower_case(String::from("Hello")), String::from("hello"));
    assert_eq!(to_lower_case(String::from("here")), String::from("here"));
    assert_eq!(
        to_lower_case(String::from("LOVELY")),
        String::from("lovely")
    );
}
