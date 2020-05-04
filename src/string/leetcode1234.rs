// https://leetcode.com/problems/replace-the-substring-for-balanced-string/
pub fn balanced_string(s: String) -> i32 {
    todo!()
}
// two_pointers string
#[test]
#[ignore]
fn test2_1234() {
    assert_eq!(balanced_string(String::from("QWER")), 0);
    assert_eq!(balanced_string(String::from("QQWE")), 1);
    assert_eq!(balanced_string(String::from("QQQW")), 2);
    assert_eq!(balanced_string(String::from("QQQQ")), 3);
}