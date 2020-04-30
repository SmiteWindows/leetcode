// https://leetcode.com/problems/excel-sheet-column-title/
pub fn convert_to_title(n: i32) -> String {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_168() {
    assert_eq!(convert_to_title(1), String::from("A"));
    assert_eq!(convert_to_title(28), String::from("AB"));
    assert_eq!(convert_to_title(701), String::from("ZY"));
}
