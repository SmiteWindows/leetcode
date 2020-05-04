// https://leetcode.com/problems/regular-expression-matching/
pub fn is_match(s: String, p: String) -> bool {
    todo!()
}
// string dynamic_programming backtracking
#[test]
#[ignore]
fn test3_10() {
    assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    assert_eq!(is_match(String::from("aa"), String::from("a*")), true);
    assert_eq!(is_match(String::from("ab"), String::from(".*")), true);
    assert_eq!(is_match(String::from("aab"), String::from("c*a*b")), true);
    assert_eq!(
        is_match(String::from("mississippi"), String::from("mis*is*p*.")),
        false
    );
}