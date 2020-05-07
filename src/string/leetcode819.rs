// https://leetcode.com/problems/most-common-word/
pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_819() {
    assert_eq!(
        most_common_word(
            String::from("Bob hit a ball, the hit BALL flew far after it was hit."),
            vec![String::from("hit")]
        ),
        String::from("ball")
    );
}
