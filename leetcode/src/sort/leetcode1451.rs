// https://leetcode-cn.com/problems/rearrange-words-in-a-sentence/
// Runtime: 12 ms
// Memory Usage: 2.9 MB
pub fn arrange_words(text: String) -> String {
    let mut words = text
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| (s.len(), i, s.to_lowercase()))
        .collect::<Vec<_>>();
    words.sort();
    let words = words.into_iter().map(|(_, _, s)| s).collect::<Vec<_>>();
    let mut res = words.join(" ");
    res[0..1].make_ascii_uppercase();
    res
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
