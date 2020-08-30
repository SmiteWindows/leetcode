// https://leetcode-cn.com/problems/longest-palindromic-substring/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::iter::FromIterator;
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    if n == 0 {
        return "".to_string();
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut start = 0;
    let mut end = 0;
    for i in 0..n {
        let mut left = i;
        let mut right = i;
        while right + 1 < n && s[right + 1] == s[left] {
            right += 1;
        }
        while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
            left -= 1;
            right += 1;
        }
        if right - left > end - start {
            start = left;
            end = right;
        }
    }
    String::from_iter(&s[start..=end])
}
// string dynamic_programming
#[test]
fn test2_5() {
    assert_eq!(
        longest_palindrome(String::from("babad")),
        String::from("bab")
    );
    assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
}
