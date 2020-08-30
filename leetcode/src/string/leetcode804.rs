// https://leetcode-cn.com/problems/unique-morse-code-words/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let map = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let mut morse = HashSet::new();
    for w in words {
        let mut s = "".to_string();
        for c in w.chars() {
            let m = map[(c as u8 - b'a') as usize];
            s += m;
        }
        morse.insert(s);
    }
    morse.len() as i32
}
// string
#[test]
fn test1_804() {
    assert_eq!(
        unique_morse_representations(vec![
            String::from("gin"),
            String::from("zen"),
            String::from("gig"),
            String::from("msg")
        ]),
        2
    );
}
