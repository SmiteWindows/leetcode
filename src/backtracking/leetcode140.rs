// https://leetcode.com/problems/word-break-ii/
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    todo!()
}
// backtracking dynamic_programming
#[test]
#[ignore]
fn test1_140() {
    assert_eq!(
        word_break(
            String::from("catsanddog"),
            vec![
                String::from("cat"),
                String::from("cats"),
                String::from("and"),
                String::from("sand"),
                String::from("dog")
            ]
        ),
        vec![String::from("cats and dog"), String::from("cat sand dog")]
    );
    assert_eq!(
        word_break(
            String::from("pineapplepenapple"),
            vec![
                String::from("apple"),
                String::from("pen"),
                String::from("applepen"),
                String::from("pine"),
                String::from("pineapple")
            ]
        ),
        vec![
            String::from("pine apple pen apple"),
            String::from("pineapple pen apple"),
            String::from("pine applepen apple")
        ]
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
        vec![String::new()]
    );
}
