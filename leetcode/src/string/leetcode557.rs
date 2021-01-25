// https://leetcode-cn.com/problems/reverse-words-in-a-string-iii/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn reverse_words(s: String) -> String {
    let words = s
        .split_whitespace()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    words.join(" ")
}
// string
#[test]
fn test1_557() {
    assert_eq!(
        reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
}
