// https://leetcode.com/problems/keyboard-row/
pub fn find_words(words: Vec<String>) -> Vec<String> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_500() {
    assert_eq!(
        find_words(vec![
            String::from("Hello"),
            String::from("Alaska"),
            String::from("Dad"),
            String::from("Peace")
        ]),
        vec![String::from("Alaska"), String::from("Dad")]
    );
}
