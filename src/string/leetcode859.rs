// https://leetcode.com/problems/buddy-strings/
pub fn buddy_strings(a: String, b: String) -> bool {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_859() {
    assert_eq!(buddy_strings(String::from("ab"), String::from("ba")), true);
    assert_eq!(buddy_strings(String::from("ab"), String::from("ab")), false);
    assert_eq!(buddy_strings(String::from("aa"), String::from("aa")), true);
    assert_eq!(
        buddy_strings(String::from("aaaaaaabc"), String::from("aaaaaaacb")),
        true
    );
    assert_eq!(buddy_strings(String::from(""), String::from("aa")), false);
}
