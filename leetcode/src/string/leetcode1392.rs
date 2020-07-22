// https://leetcode.com/problems/longest-happy-prefix/
pub fn longest_prefix(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1392() {
    assert_eq!(longest_prefix(String::from("level")), String::from("l"));
    assert_eq!(longest_prefix(String::from("ababab")), String::from("abab"));
    assert_eq!(
        longest_prefix(String::from("leetcodeleet")),
        String::from("leet")
    );
    assert_eq!(longest_prefix(String::from("a")), String::from(""));
}
