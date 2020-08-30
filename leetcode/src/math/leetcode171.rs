// https://leetcode-cn.com/problems/excel-sheet-column-number/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn title_to_number(s: String) -> i32 {
    s.bytes()
        .fold(0, |sum, c| sum * 26 + i32::from(c) - 'A' as i32 + 1)
}
// math
#[test]
fn test1_171() {
    assert_eq!(title_to_number(String::from("A")), 1);
    assert_eq!(title_to_number(String::from("AB")), 28);
    assert_eq!(title_to_number(String::from("ZY")), 701);
}
