// https://leetcode-cn.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    sentence
        .split_whitespace()
        .enumerate()
        .position(|(i, word)| word.starts_with(&search_word))
        .map_or(-1, |x| (x + 1) as i32)
}
// string
#[test]
fn test1_1455() {
    assert_eq!(
        is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
        4
    );
    assert_eq!(
        is_prefix_of_word(
            "this problem is an easy problem".to_string(),
            "pro".to_string()
        ),
        2
    );
    assert_eq!(
        is_prefix_of_word("i am tired".to_string(), "you".to_string()),
        -1
    );
    assert_eq!(
        is_prefix_of_word("i use triple pillow".to_string(), "pill".to_string()),
        4
    );
    assert_eq!(
        is_prefix_of_word("hello from the other side".to_string(), "they".to_string()),
        -1
    );
}
