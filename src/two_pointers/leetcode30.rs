// https://leetcode.com/problems/substring-with-concatenation-of-all-words/
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    todo!()
}
// string two_pointers hash_table
#[test]
#[ignore]
fn test1_30() {
    assert_eq!(
        find_substring(
            String::from("barfoothefoobarman"),
            vec![String::from("foo"), String::from("bar")]
        ),
        vec![0, 9]
    );
    assert_eq!(
        find_substring(
            String::from("wordgoodgoodgoodbestword"),
            vec![
                String::from("word"),
                String::from("good"),
                String::from("best"),
                String::from("word")
            ]
        ),
        vec![]
    );
}
