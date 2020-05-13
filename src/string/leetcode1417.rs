// https://leetcode.com/problems/reformat-the-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::mem::swap;
pub fn reformat(s: String) -> String {
    let mut chars: Vec<char> = vec![];
    let mut digits: Vec<char> = vec![];
    let mut res: Vec<char> = vec![];
    for c in s.chars() {
        if c.is_digit(10) {
            digits.push(c);
        } else {
            chars.push(c);
        }
    }
    let mut iter;
    let mut next_iter;
    if digits.len() >= chars.len() {
        if digits.len() > chars.len() + 1 {
            return "".to_string();
        } else {
            iter = digits.iter();
            next_iter = chars.iter();
        }
    } else if chars.len() > digits.len() + 1 {
        return "".to_string();
    } else {
        iter = chars.iter();
        next_iter = digits.iter();
    }

    while let Some(&c) = iter.next() {
        res.push(c);
        swap(&mut iter, &mut next_iter);
    }
    res.into_iter().collect()
}
// string
#[test]
fn test1_1417() {
    assert_eq!(reformat(String::from("a0b1c2")), String::from("0a1b2c"));
    assert_eq!(reformat(String::from("leetcode")), String::from(""));
    assert_eq!(reformat(String::from("1229857369")), String::from(""));
    assert_eq!(
        reformat(String::from("covid2019")),
        String::from("c2o0v1i9d")
    );
    assert_eq!(reformat(String::from("ab123")), String::from("1a2b3"));
}
