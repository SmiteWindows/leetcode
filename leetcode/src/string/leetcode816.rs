// https://leetcode-cn.com/problems/ambiguous-coordinates/
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
    use leetcode_prelude::vec_string;
    assert_eq!(
        ambiguous_coordinates("(123)".to_string()),
        vec_string!["(1, 2.3)", "(1, 23)", "(1.2, 3)", "(12, 3)"]
    );
    assert_eq!(
        ambiguous_coordinates("(00011)".to_string()),
        vec_string!["(0, 0.011)", "(0.001, 1)"]
    );
    assert_eq!(
        ambiguous_coordinates("(0123)".to_string()),
        vec_string![
            "(0, 1.23)",
            "(0, 12.3)",
            "(0, 123)",
            "(0.1, 2.3)",
            "(0.1, 23)",
            "(0.12, 3)"
        ]
    );
    assert_eq!(
        ambiguous_coordinates("(100)".to_string()),
        vec_string!["(10, 0)"]
    );
}
