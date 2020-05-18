// https://leetcode.com/problems/rearrange-words-in-a-sentence/
pub fn arrange_words(text: String) -> String {
    todo!()
}
// string sort
#[test]
fn test2_1451() {
    assert_eq!(
        arrange_words(String::from("Leetcode is cool")),
        String::from("Is cool leetcode")
    );
    assert_eq!(
        arrange_words(String::from("Keep calm and code on")),
        String::from("On and keep calm code")
    );
    assert_eq!(
        arrange_words(String::from("To be or not to be")),
        String::from("To be or to be not")
    );
}
