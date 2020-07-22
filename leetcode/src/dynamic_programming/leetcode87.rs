// https://leetcode.com/problems/scramble-string/
pub fn is_scramble(s1: String, s2: String) -> bool {
    todo!()
}
// dynamic_programming string
#[test]
#[ignore]
fn test1_87() {
    assert_eq!(
        is_scramble(String::from("great"), String::from("rgeat")),
        true
    );
    assert_eq!(
        is_scramble(String::from("abcde"), String::from("caebd")),
        false
    );
}
