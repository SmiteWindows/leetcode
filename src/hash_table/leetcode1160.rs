// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut hs: HashMap<char, i32> = HashMap::new();
    for c in chars.chars() {
        *hs.entry(c).or_default() += 1;
    }
    let mut sum = 0;
    for w in words {
        let mut hs=hs.clone();
        let mut valid = true;
        for c in w.chars() {
            let count = hs.entry(c).or_default();
            *count -= 1;
            if *count < 0 {
                valid = false;
                break;
            }
        }
        if valid {
            sum += w.len();
        }
    }
    sum as i32
}
// hash_table array
#[test]
fn test1_1160() {
    assert_eq!(
        count_characters(
            vec![
                String::from("cat"),
                String::from("bt"),
                String::from("hat"),
                String::from("tree")
            ],
            String::from("atach")
        ),
        6
    );
    assert_eq!(
        count_characters(
            vec![
                String::from("hello"),
                String::from("world"),
                String::from("leetcode")
            ],
            String::from("welldonehoneyr")
        ),
        10
    );
}
