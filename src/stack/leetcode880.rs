// https://leetcode.com/problems/decoded-string-at-index/
pub fn decode_at_index(s: String, k: i32) -> String {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_880() {
    assert_eq!(
        decode_at_index(String::from("leet2code3"), 10),
        String::from("o")
    );
    assert_eq!(decode_at_index(String::from("ha22"), 5), String::from("h"));
    assert_eq!(
        decode_at_index(String::from("a2345678999999999999999"), 1),
        String::from("a")
    );
}
