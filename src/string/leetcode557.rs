// https://leetcode.com/problems/reverse-words-in-a-string-iii/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn reverse_words(s: String) -> String {
    let words: Vec<String> = s
        .split_whitespace()
        .map(|s| s.chars().rev().collect())
        .collect();
    let res: String = words.join(" ");
    res
}
// string
#[test]
fn test1_557() {
    assert_eq!(
        reverse_words(String::from("Let's take LeetCode contest")),
        String::from("s'teL ekat edoCteeL tsetnoc")
    );
}
