// https://leetcode.com/problems/reformat-the-string/
pub fn reformat(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1417() {
    assert_eq!(reformat(String::from("a0b1c2")), String::from("0a1b2c"));
    assert_eq!(reformat(String::from("leetcode")), String::from(""));
    assert_eq!(reformat(String::from("1229857369")), String::from(""));
    assert_eq!(
        reformat(String::from("covid2019")),
        String::from("c2o0v1i9d")
    );
    assert_eq!(reformat(String::from("ab123")), String::from("1a2b3"));
}
