// https://leetcode.com/problems/isomorphic-strings/
pub fn is_isomorphic(s: String, t: String) -> bool {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_205() {
    assert_eq!(
        is_isomorphic(String::from("egg"), String::from("add")),
        true
    );
    assert_eq!(
        is_isomorphic(String::from("foo"), String::from("bar")),
        false
    );
    assert_eq!(
        is_isomorphic(String::from("paper"), String::from("title")),
        true
    );
}
