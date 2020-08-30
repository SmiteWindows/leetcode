// https://leetcode-cn.com/problems/longest-word-in-dictionary-through-deleting/
pub fn find_longest_word(s: String, d: Vec<String>) -> String {
    todo!()
}
// sort two_pointers
#[test]
#[ignore]
fn test1_524() {
    assert_eq!(
        find_longest_word(
            String::from("abpcplea"),
            vec![
                String::from("ale"),
                String::from("apple"),
                String::from("monkey"),
                String::from("plea")
            ]
        ),
        String::from("apple")
    );
    assert_eq!(
        find_longest_word(
            String::from("abpcplea"),
            vec![String::from("a"), String::from("b"), String::from("c")]
        ),
        String::from("a")
    );
}
