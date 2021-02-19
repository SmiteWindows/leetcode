// https://leetcode-cn.com/problems/excel-sheet-column-title/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn convert_to_title(mut n: i32) -> String {
    let mut v = Vec::new();
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
    assert_eq!(convert_to_title(1), "A".to_string());
    assert_eq!(convert_to_title(28), "AB".to_string());
    assert_eq!(convert_to_title(701), "ZY".to_string());
}
