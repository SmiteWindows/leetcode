// https://leetcode-cn.com/problems/check-if-string-is-transformable-with-substring-sort-operations/
pub fn is_transformable(s: String, t: String) -> bool {
    todo!()
}
// string greedy
#[test]
#[ignore]
fn test1_1585() {
    assert_eq!(
        is_transformable("84532".to_string(), "34852".to_string()),
        true
    );
    assert_eq!(
        is_transformable("34521".to_string(), "23415".to_string()),
        true
    );
    assert_eq!(
        is_transformable("12345".to_string(), "12435".to_string()),
        false
    );
    assert_eq!(is_transformable("1".to_string(), "2".to_string()), false);
}
