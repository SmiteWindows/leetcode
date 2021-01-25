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
        reverse_words("the sky is blue".to_string()),
        "blue is sky the".to_string()
    );
    assert_eq!(
        reverse_words("  hello world!  ".to_string()),
        "world! hello".to_string()
    );
    assert_eq!(
        reverse_words("a good   example".to_string()),
        "example good a".to_string()
    );
}
