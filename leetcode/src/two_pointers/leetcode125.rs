// https://leetcode-cn.com/problems/valid-palindrome/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn is_palindrome(s: String) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>();
    let a = s.iter().collect::<String>();
    let b = s.iter().rev().collect::<String>();
    a == b
}
// string two_pointers
#[test]
fn test1_125() {
    assert_eq!(
        is_palindrome(String::from("A man, a plan, a canal: Panama")),
        true
    );
    assert_eq!(is_palindrome(String::from("race a car")), false);
}
