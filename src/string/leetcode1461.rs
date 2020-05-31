// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
pub fn has_all_codes(s: String, k: i32) -> bool {
    todo!()
}
// string bit_manipulation
#[test]
#[ignore]
fn test1_1461() {
    assert_eq!(has_all_codes(String::from("00110110"), 2), true);
    assert_eq!(has_all_codes(String::from("00110"), 2), true);
    assert_eq!(has_all_codes(String::from("0110"), 1), true);
    assert_eq!(has_all_codes(String::from("0110"), 2), false);
    assert_eq!(has_all_codes(String::from("0000000001011100"), 4), false);
}
