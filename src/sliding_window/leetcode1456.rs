// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
pub fn max_vowels(s: String, k: i32) -> i32 {
    todo!()
}
// string sliding_window
#[test]
#[ignore]
fn test2_1456() {
    assert_eq!(max_vowels(String::from("abciiidef"), 3), 3);
    assert_eq!(max_vowels(String::from("aeiou"), 2), 2);
    assert_eq!(max_vowels(String::from("leetcode"), 3), 2);
    assert_eq!(max_vowels(String::from("rhythms"), 4), 0);
    assert_eq!(max_vowels(String::from("tryhard"), 4), 1);
}
