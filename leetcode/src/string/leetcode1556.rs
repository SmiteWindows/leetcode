// https://leetcode-cn.com/problems/thousand-separator/
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
// Runtime: 0 ms
// Memory Usage: 2 MB
// ✔✔
// string
#[test]
fn test1_1556() {
    assert_eq!(thousand_separator(987), "987");
    assert_eq!(thousand_separator(1234), "1.234");
    assert_eq!(thousand_separator(123456789), "123.456.789");
    assert_eq!(thousand_separator(0), "0");
}
fn _thousand_separator(mut n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut stack = Vec::new();
    let mut count = 0;
    while n > 0 {
        if count % 3 == 0 && count > 0 {
            stack.push('.');
        }
        stack.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
        count += 1;
    }
    stack.into_iter().rev().collect()
}
