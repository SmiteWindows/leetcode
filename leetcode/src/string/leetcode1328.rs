// https://leetcode-cn.com/problems/break-a-palindrome/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn break_palindrome(palindrome: String) -> String {
    let n = palindrome.len();
    let mut s = palindrome.chars().collect::<Vec<_>>();
    if n == 1 {
        return "".to_string();
    }
    for i in 0..n / 2 {
        if s[i] > 'a' {
            s[i] = 'a';
            return s.into_iter().collect();
        }
    }
    s[n - 1] = 'b';
    s.into_iter().collect()
}
// string
#[test]
fn test1_1328() {
    assert_eq!(
        break_palindrome("abccba")),
        "aaccba")
    );
    assert_eq!(break_palindrome("a")), ""));
}
