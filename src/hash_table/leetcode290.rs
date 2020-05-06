// https://leetcode.com/problems/word-pattern/
pub fn word_pattern(pattern: String, str: String) -> bool {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_290() {
    assert_eq!(
        word_pattern(String::from("abba"), String::from("dog cat cat dog")),
        true
    );
    assert_eq!(
        word_pattern(String::from("abba"), String::from("dog cat cat fish")),
        false
    );
    assert_eq!(
        word_pattern(String::from("aaaa"), String::from("dog cat cat dog")),
        false
    );
    assert_eq!(
        word_pattern(String::from("abba"), String::from("dog dog dog dog")),
        false
    );
}
