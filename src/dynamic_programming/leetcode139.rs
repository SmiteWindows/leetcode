// https://leetcode.com/problems/word-break/
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_139() {
    assert_eq!(
        word_break(
            String::from("leetcode"),
            vec![String::from("leet"), String::from("code")]
        ),
        true
    );
    assert_eq!(
        word_break(
            String::from("applepenapple"),
            vec![String::from("apple"), String::from("pen")]
        ),
        true
    );
    assert_eq!(
        word_break(
            String::from("catsandog"),
            vec![
                String::from("cats"),
                String::from("dog"),
                String::from("sand"),
                String::from("and"),
                String::from("cat")
            ]
        ),
        false
    );
}
