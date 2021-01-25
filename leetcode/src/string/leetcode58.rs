// https://leetcode-cn.com/problems/length-of-last-word/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn length_of_last_word(s: String) -> i32 {
    if let Some(last) = s.split_whitespace().last() {
        last.len() as i32
    } else {
        0
    }
}
// string
#[test]
fn test1_58() {
    assert_eq!(length_of_last_word("Hello World".to_string()), 5);
}
