// https://leetcode.com/problems/break-a-palindrome/
pub fn break_palindrome(palindrome: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1328() {
    assert_eq!(
        break_palindrome(String::from("abccba")),
        String::from("aaccba")
    );
    assert_eq!(break_palindrome(String::from("a")), String::from(""));
}
