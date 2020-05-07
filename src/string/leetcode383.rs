// https://leetcode.com/problems/ransom-note/
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_383() {
    assert_eq!(can_construct(String::from("a"), String::from("b")), false);
    assert_eq!(can_construct(String::from("aa"), String::from("ab")), false);
    assert_eq!(can_construct(String::from("aa"), String::from("aab")), true);
}
