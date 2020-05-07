// https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/
pub fn query_string(s: String, n: i32) -> bool {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1016() {
    assert_eq!(query_string(String::from("0110"), 3), true);
    assert_eq!(query_string(String::from("0110"), 4), false);
}
