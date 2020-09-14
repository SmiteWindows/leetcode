// https://leetcode-cn.com/problems/check-if-string-is-transformable-with-substring-sort-operations/
pub fn is_transformable(s: String, t: String) -> bool {
    todo!()
}
// string greedy
#[test]
#[ignore]
fn test2_1585() {
    assert_eq!(
        is_transformable(String::from("84532"), String::from("34852")),
        true
    );
    assert_eq!(
        is_transformable(String::from("34521"), String::from("23415")),
        true
    );
    assert_eq!(
        is_transformable(String::from("12345"), String::from("12435")),
        false
    );
    assert_eq!(
        is_transformable(String::from("1"), String::from("2")),
        false
    );
}
