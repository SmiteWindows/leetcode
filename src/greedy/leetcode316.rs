// https://leetcode.com/problems/remove-duplicate-letters/
pub fn remove_duplicate_letters(s: String) -> String {
    todo!()
}
// stack greedy
#[test]
#[ignore]
fn test2_316() {
    assert_eq!(
        remove_duplicate_letters(String::from("bcabc")),
        String::from("abc")
    );
    assert_eq!(
        remove_duplicate_letters(String::from("cbacdcbc")),
        String::from("acdb")
    );
}
