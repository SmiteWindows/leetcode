// https://leetcode.com/problems/valid-anagram/
pub fn is_anagram(s: String, t: String) -> bool {
    todo!()
}
// sort hash_table
#[test]
#[ignore]
fn test2_242() {
    assert_eq!(
        is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
}
