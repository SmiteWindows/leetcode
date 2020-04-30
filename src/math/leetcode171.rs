// https://leetcode.com/problems/excel-sheet-column-number/
pub fn title_to_number(s: String) -> i32 {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_171() {
    assert_eq!(title_to_number(String::from("A")), 1);
    assert_eq!(title_to_number(String::from("AB")), 28);
    assert_eq!(title_to_number(String::from("ZY")), 701);
}
