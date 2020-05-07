// https://leetcode.com/problems/reverse-words-in-a-string/
pub fn reverse_words(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_151() {
    assert_eq!(
        reverse_words(String::from("the sky is blue")),
        String::from("blue is sky the")
    );
    assert_eq!(
        reverse_words(String::from("  hello world!  ")),
        String::from("world! hello")
    );
    assert_eq!(
        reverse_words(String::from("a good   example")),
        String::from("example good a")
    );
}
