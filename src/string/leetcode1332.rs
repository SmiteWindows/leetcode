// https://leetcode.com/problems/remove-palindromic-subsequences/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn remove_palindrome_sub(s: String) -> i32 {
    if s.is_empty() {
        0
    } else {
        if is_palindrome(&s) {
            1
        } else {
            2
        }
    }
}

fn is_palindrome(s: &str) -> bool {
    let mut i = 0;
    let n = s.len();
    let mut j = n - 1;
    let v = s.chars().collect::<Vec<_>>();
    while i <= j {
        if v[i] != v[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
// string
#[test]
fn test1_1332() {
    assert_eq!(remove_palindrome_sub(String::from("ababa")), 1);
    assert_eq!(remove_palindrome_sub(String::from("abb")), 2);
    assert_eq!(remove_palindrome_sub(String::from("baabb")), 2);
    assert_eq!(remove_palindrome_sub(String::from("")), 0);
}
