// https://leetcode.com/problems/wildcard-matching/
pub fn is_match(s: String, p: String) -> bool {
    todo!()
}
// string dynamic_programming backtracking greedy
#[test]
#[ignore]
fn test4_44() {
    assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    assert_eq!(is_match(String::from("aa"), String::from("*")), true);
    assert_eq!(is_match(String::from("cb"), String::from("?a")), false);
    assert_eq!(is_match(String::from("adceb"), String::from("a*b")), true);
    assert_eq!(
        is_match(String::from("acdcb"), String::from("a*c?b")),
        false
    );
}