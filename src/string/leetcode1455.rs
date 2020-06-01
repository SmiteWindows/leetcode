// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
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
        is_prefix_of_word(String::from("i love eating burger"), String::from("burg")),
        4
    );
    assert_eq!(
        is_prefix_of_word(
            String::from("this problem is an easy problem"),
            String::from("pro")
        ),
        2
    );
    assert_eq!(
        is_prefix_of_word(String::from("i am tired"), String::from("you")),
        -1
    );
    assert_eq!(
        is_prefix_of_word(String::from("i use triple pillow"), String::from("pill")),
        4
    );
    assert_eq!(
        is_prefix_of_word(
            String::from("hello from the other side"),
            String::from("they")
        ),
        -1
    );
}
