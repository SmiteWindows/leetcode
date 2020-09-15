// https://leetcode-cn.com/problems/reorder-data-in-log-files/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
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
    use leetcode_prelude::vec_string;
    assert_eq!(
        reorder_log_files(vec_string![
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero"
        ]),
        vec![
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6"
        ]
    );
}
