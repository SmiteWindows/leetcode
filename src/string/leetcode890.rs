// https://leetcode.com/problems/find-and-replace-pattern/
pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_890() {
    assert_eq!(
        find_and_replace_pattern(
            vec![
                String::from("abc"),
                String::from("deq"),
                String::from("mee"),
                String::from("aqq"),
                String::from("dkd"),
                String::from("ccc")
            ],
            String::from("abb")
        ),
        vec![String::from("mee"), String::from("aqq")]
    );
}
