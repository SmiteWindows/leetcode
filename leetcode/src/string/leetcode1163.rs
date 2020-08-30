// https://leetcode-cn.com/problems/last-substring-in-lexicographical-order/
pub fn last_substring(s: String) -> String {
    todo!()
}
// string suffix_array
#[test]
#[ignore]
fn test2_1163() {
    assert_eq!(last_substring(String::from("abab")), String::from("bab"));
    assert_eq!(
        last_substring(String::from("leetcode")),
        String::from("tcode")
    );
}
