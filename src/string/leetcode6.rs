// https://leetcode.com/problems/zigzag-conversion/
pub fn convert(s: String, num_rows: i32) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_6() {
    assert_eq!(
        convert(String::from("PAYPALISHIRING"), 3),
        String::from("PAHNAPLSIIGYIR")
    );
    assert_eq!(
        convert(String::from("PAYPALISHIRING"), 4),
        String::from("PINALSIGYAHRPI")
    );
}
