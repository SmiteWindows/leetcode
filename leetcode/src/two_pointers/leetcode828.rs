// https://leetcode-cn.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
pub fn unique_letter_string(s: String) -> i32 {
    todo!()
}
// two_pointers
#[test]
#[ignore]
fn test1_828() {
    assert_eq!(unique_letter_string("ABC".to_string()), 10);
    assert_eq!(unique_letter_string("ABA".to_string()), 8);
    assert_eq!(unique_letter_string("LEETCODE".to_string()), 92);
}
