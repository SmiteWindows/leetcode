// https://leetcode-cn.com/problems/valid-palindrome-ii/
// Runtime: 4 ms
// Memory Usage: 3.8 MB
pub fn valid_palindrome(s: String) -> bool {
    if let Some(s) = is_palidrome(&s) {
        let sl = &s[1..];
        let sr = &s[..s.len() - 1];
        is_palidrome(sl).is_none() || is_palidrome(sr).is_none()
    } else {
        true
    }
}

fn is_palidrome(v: &str) -> Option<&str> {
    let n = v.len();
    if n <= 1 {
        return None;
    }
    if v.chars().next() == v.chars().last() {
        is_palidrome(&v[1..n - 1])
    } else {
        Some(v)
    }
}
// string
#[test]
fn test1_680() {
    // assert_eq!(valid_palindrome("aba")), true);
    assert_eq!(valid_palindrome("abca".to_string()), true);
}
