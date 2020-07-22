// https://leetcode.com/problems/ambiguous-coordinates/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::iter::FromIterator;
pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut res = vec![];
    for i in 2..n - 1 {
        for l in nums(&s[1..i]) {
            for r in nums(&s[i..n - 1]) {
                res.push(format!("({}, {})", l, r));
            }
        }
    }
    res
}

fn nums(ch: &[char]) -> Vec<String> {
    let n = ch.len();
    let mut res = vec![];
    for i in 1..=n {
        let left = String::from_iter(ch[..i].iter());
        let right = String::from_iter(ch[i..].iter());
        if left.starts_with('0') && left != "0" {
            continue;
        }
        if right.ends_with('0') {
            continue;
        }
        res.push(format!(
            "{}{}{}",
            left,
            if i == n { "" } else { "." },
            right
        ));
    }
    res
}
// string
#[test]
fn test1_816() {
    assert_eq!(
        ambiguous_coordinates(String::from("(123)")),
        vec![
            String::from("(1, 2.3)"),
            String::from("(1, 23)"),
            String::from("(1.2, 3)"),
            String::from("(12, 3)"),
        ]
    );
    assert_eq!(
        ambiguous_coordinates(String::from("(00011)")),
        vec![String::from("(0, 0.011)"), String::from("(0.001, 1)")]
    );
    assert_eq!(
        ambiguous_coordinates(String::from("(0123)")),
        vec![
            String::from("(0, 1.23)"),
            String::from("(0, 12.3)"),
            String::from("(0, 123)"),
            String::from("(0.1, 2.3)"),
            String::from("(0.1, 23)"),
            String::from("(0.12, 3)")
        ]
    );
    assert_eq!(
        ambiguous_coordinates(String::from("(100)")),
        vec![String::from("(10, 0)")]
    );
}
