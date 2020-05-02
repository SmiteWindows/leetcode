// https://leetcode.com/problems/replace-words/
pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
    todo!()
}
// hash_table trie
#[test]
#[ignore]
fn test2_648() {
    assert_eq!(
        replace_words(
            vec![
                String::from("cat"),
                String::from("bat"),
                String::from("rat"),
            ],
            String::from("the cattle was rattled by the battery")
        ),
        String::from("the cat was rat by the bat"),
    );
}
