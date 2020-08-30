// https://leetcode-cn.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
pub fn unique_letter_string(s: String) -> i32 {
    todo!()
}
// two_pointers
#[test]
#[ignore]
fn test1_828() {
    assert_eq!(unique_letter_string(String::from("ABC")), 10);
    assert_eq!(unique_letter_string(String::from("ABA")), 8);
    assert_eq!(unique_letter_string(String::from("LEETCODE")), 92);
}
