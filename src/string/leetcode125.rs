// https://leetcode.com/problems/valid-palindrome/
pub fn is_palindrome(s: String) -> bool {
    todo!()
}
// string two_pointers
#[test]
#[ignore]
fn test2_125() {
    assert_eq!(
        is_palindrome(String::from("A man, a plan, a canal: Panama")),
        true
    );
    assert_eq!(is_palindrome(String::from("race a car")), false);
}
