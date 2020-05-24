// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
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
