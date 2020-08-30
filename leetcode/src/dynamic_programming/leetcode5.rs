// https://leetcode-cn.com/problems/longest-palindromic-substring/
pub fn longest_palindrome(s: String) -> String {
    todo!()
}
// string dynamic_programming
#[test]
#[ignore]
fn test1_5() {
    assert_eq!(
        longest_palindrome(String::from("babad")),
        String::from("bab")
    );
    assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
}
