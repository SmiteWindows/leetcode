// https://leetcode.com/problems/reorder-data-in-log-files/
pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
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
