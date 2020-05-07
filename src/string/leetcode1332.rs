// https://leetcode.com/problems/remove-palindromic-subsequences/
pub fn remove_palindrome_sub(s: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1332() {
    assert_eq!(remove_palindrome_sub(String::from("ababa")), 1);
    assert_eq!(remove_palindrome_sub(String::from("abb")), 2);
    assert_eq!(remove_palindrome_sub(String::from("baabb")), 2);
    assert_eq!(remove_palindrome_sub(String::from("")), 0);
}
