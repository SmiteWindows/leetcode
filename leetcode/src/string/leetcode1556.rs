// https://leetcode.com/problems/thousand-separator/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
// ✔
pub fn thousand_separator(n: i32) -> String {
    let mut characters: Vec<_> = n.to_string().chars().rev().collect();
    let mut count = 0;
    let mut i = 0;
    while i < characters.len() {
        if count == 3 {
            characters.insert(i, '.');
            count = -1;
        }
        count += 1;
        i += 1;
    }
    characters.into_iter().rev().collect()
}
// string
#[test]
fn test1_1556() {
    assert_eq!(thousand_separator(987), "987");
    assert_eq!(thousand_separator(1234), "1.234");
    assert_eq!(thousand_separator(123456789), "123.456.789");
    assert_eq!(thousand_separator(0), "0");
}
