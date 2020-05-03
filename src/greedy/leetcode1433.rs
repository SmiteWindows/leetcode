// https://leetcode.com/problems/check-if-a-string-can-break-another-string/
pub fn check_if_can_break(s1: String, s2: String) -> bool {
    todo!()
}
// string greedy
#[test]
#[ignore]
fn test2_1433() {
    assert_eq!(
        check_if_can_break(String::from("abc"), String::from("xya")),
        true
    );
    assert_eq!(
        check_if_can_break(String::from("abe"), String::from("acd")),
        false
    );
    assert_eq!(
        check_if_can_break(String::from("leetcodee"), String::from("interview")),
        true
    );
}
