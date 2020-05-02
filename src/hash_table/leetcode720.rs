// https://leetcode.com/problems/longest-word-in-dictionary/
pub fn longest_word(words: Vec<String>) -> String {
    todo!()
}
// hash_table trie
#[test]
#[ignore]
fn test2_720() {
    assert_eq!(
        longest_word(vec![
            String::from("w"),
            String::from("wo"),
            String::from("wor"),
            String::from("worl"),
            String::from("world"),
        ]),
        String::from("world")
    );
    assert_eq!(
        longest_word(vec![
            String::from("a"),
            String::from("banana"),
            String::from("app"),
            String::from("appl"),
            String::from("ap"),
            String::from("apply"),
            String::from("apple"),
        ]),
        String::from("apple"),
    );
}
