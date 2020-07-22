// https://leetcode.com/problems/distinct-echo-substrings/
pub fn distinct_echo_substrings(text: String) -> i32 {
    todo!()
}
// string rolling_hash
#[test]
#[ignore]
fn test1_1316() {
    assert_eq!(distinct_echo_substrings(String::from("abcabcabc")), 3);
    assert_eq!(
        distinct_echo_substrings(String::from("leetcodeleetcode")),
        2
    );
}
