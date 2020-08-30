// https://leetcode-cn.com/problems/reverse-words-in-a-string/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
// string
#[test]
fn test1_151() {
    assert_eq!(
        reverse_words(String::from("the sky is blue")),
        String::from("blue is sky the")
    );
    assert_eq!(
        reverse_words(String::from("  hello world!  ")),
        String::from("world! hello")
    );
    assert_eq!(
        reverse_words(String::from("a good   example")),
        String::from("example good a")
    );
}
