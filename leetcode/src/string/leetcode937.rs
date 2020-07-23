// https://leetcode.com/problems/reorder-data-in-log-files/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
    let mut logs = logs;
    logs.sort_by(|a, b| {
        let i = a.find(' ').unwrap();
        let j = b.find(' ').unwrap();
        let ar = &a[i + 1..];
        let br = &b[j + 1..];
        let ac = a.chars().last().unwrap();
        let bc = b.chars().last().unwrap();
        match (ac.is_digit(10), bc.is_digit(10)) {
            (true, true) => Equal,
            (true, false) => Greater,
            (false, true) => Less,
            (false, false) => {
                let ordering = ar.cmp(br);
                if ordering == Equal {
                    a.cmp(b)
                } else {
                    ordering
                }
            }
        }
    });
    logs
}
// string
#[test]
fn test1_937() {
    assert_eq!(
        reorder_log_files(vec![
            String::from("dig1 8 1 5 1"),
            String::from("let1 art can"),
            String::from("dig2 3 6"),
            String::from("let2 own kit dig"),
            String::from("let3 art zero")
        ]),
        vec![
            String::from("let1 art can"),
            String::from("let3 art zero"),
            String::from("let2 own kit dig"),
            String::from("dig1 8 1 5 1"),
            String::from("dig2 3 6")
        ]
    );
}
