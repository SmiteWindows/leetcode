// https://leetcode.com/problems/uncommon-words-from-two-sentences/
pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_884() {
    assert_eq!(
        uncommon_from_sentences(
            String::from("this apple is sweet"),
            String::from("this apple is sour")
        ),
        vec![String::from("sweet"), String::from("sour")]
    );
    assert_eq!(
        uncommon_from_sentences(String::from("apple apple"), String::from("banana")),
        vec![String::from("banana")]
    );
}
