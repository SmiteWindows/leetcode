// https://leetcode-cn.com/problems/reformat-the-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::mem::swap;
pub fn reformat(s: String) -> String {
    let mut chars = vec![];
    let mut digits = vec![];
    let mut res = vec![];
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
    assert_eq!(reformat("a0b1c2".to_string()), "0a1b2c".to_string());
    assert_eq!(reformat("leetcode".to_string()), "".to_string());
    assert_eq!(reformat("1229857369".to_string()), "".to_string());
    assert_eq!(reformat("covid2019".to_string()), "c2o0v1i9d".to_string());
    assert_eq!(reformat("ab123".to_string()), "1a2b3".to_string());
}
