// https://leetcode-cn.com/problems/reformat-date/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn reformat_date(date: String) -> String {
    let month = vec![
        ("Jan", 1),
        ("Feb", 2),
        ("Mar", 3),
        ("Apr", 4),
        ("May", 5),
        ("Jun", 6),
        ("Jul", 7),
        ("Aug", 8),
        ("Sep", 9),
        ("Oct", 10),
        ("Nov", 11),
        ("Dec", 12),
    ]
    .into_iter()
    .map(|(m, mm)| (m.to_string(), mm))
    .collect::<HashMap<String, i32>>();
    let mut v = date
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let yyyy = v.pop().unwrap();
    let mm = month[&v.pop().unwrap()];
    let mut dd = v.pop().unwrap();
    dd.pop();
    dd.pop();
    let dd = dd.parse::<i32>().unwrap();
    format!("{}-{:02}-{:02}", yyyy, mm, dd)
}
// string
#[test]
fn test1_1507() {
    assert_eq!(
        reformat_date("20th Oct 2052".to_string()),
        "2052-10-20".to_string()
    );
    assert_eq!(
        reformat_date("6th Jun 1933".to_string()),
        "1933-06-06".to_string()
    );
    assert_eq!(
        reformat_date("26th May 1960".to_string()),
        "1960-05-26".to_string()
    );
}
