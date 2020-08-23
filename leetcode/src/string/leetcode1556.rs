// https://leetcode.com/problems/thousand-separator/
pub fn thousand_separator(n: i32) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1556() {
    assert_eq!(thousand_separator(987), "987");
    assert_eq!(thousand_separator(1234), "1.234");
    assert_eq!(thousand_separator(123456789), "123.456.789");
    assert_eq!(thousand_separator(0), "0");
}
