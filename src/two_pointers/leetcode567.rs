// https://leetcode.com/problems/permutation-in-string/
pub fn check_inclusion(s1: String, s2: String) -> bool {
    todo!()
}
// sliding_window two_pointers
#[test]
#[ignore]
fn test2_567() {
    assert_eq!(
        check_inclusion(String::from("ab"), String::from("eidbaooo")),
        true
    );
    assert_eq!(
        check_inclusion(String::from("ab"), String::from("eidboaoo")),
        false
    );
}
