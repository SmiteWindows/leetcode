// https://leetcode.com/problems/di-string-match/
pub fn di_string_match(s: String) -> Vec<i32> {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_942() {
    assert_eq!(di_string_match(String::from("IDID")), vec![0, 4, 1, 3, 2]);
    assert_eq!(di_string_match(String::from("III")), vec![0, 1, 2, 3]);
    assert_eq!(di_string_match(String::from("DDI")), vec![3, 2, 0, 1]);
}
