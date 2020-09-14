// https://leetcode-cn.com/problems/longest-word-in-dictionary-through-deleting/
pub fn find_longest_word(s: String, d: Vec<String>) -> String {
    todo!()
}
// sort two_pointers
#[test]
#[ignore]
fn test1_524() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_longest_word(
            "abpcplea".to_string(),
            vec_string!["ale", "apple", "monkey", "plea"]
        ),
        "apple".to_string()
    );
    assert_eq!(
        find_longest_word("abpcplea".to_string(), vec_string!["a", "b", "c"]),
        "a".to_string()
    );
}
