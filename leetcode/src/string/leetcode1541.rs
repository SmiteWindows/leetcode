// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/
pub fn min_insertions(s: String) -> i32 {
    todo!()
}
// string stack
#[test]
#[ignore]
fn test1_1541() {
    assert_eq!(min_insertions("(()))".to_string()), 1);
    assert_eq!(min_insertions("())".to_string()), 0);
    assert_eq!(min_insertions("))())(".to_string()), 3);
    assert_eq!(min_insertions("((((((".to_string()), 12);
    assert_eq!(min_insertions(")))))))".to_string()), 5);
}
