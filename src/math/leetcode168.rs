// https://leetcode.com/problems/excel-sheet-column-title/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn convert_to_title(n: i32) -> String {
    let mut n = n;
    let mut v = vec![];
    while n > 0 {
        let x = ((n - 1) % 26) as u8;
        let c = (x + b'A') as char;
        v.insert(0, c);
        n = (n - 1) / 26;
    }
    v.iter().collect()
}
// math
#[test]
fn test1_168() {
    assert_eq!(convert_to_title(1), String::from("A"));
    assert_eq!(convert_to_title(28), String::from("AB"));
    assert_eq!(convert_to_title(701), String::from("ZY"));
}
