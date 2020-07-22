// https://leetcode.com/problems/decode-string/
pub fn decode_string(s: String) -> String {
    todo!()
}
// stack depth_first_search
#[test]
#[ignore]
fn test2_394() {
    assert_eq!(
        decode_string(String::from("3[a]2[bc]")),
        String::from("aaabcbc")
    );
    assert_eq!(
        decode_string(String::from("3[a2[c]]")),
        String::from("accaccacc")
    );
    assert_eq!(
        decode_string(String::from("2[abc]3[cd]ef")),
        String::from("abcabccdcdcdef")
    );
}
