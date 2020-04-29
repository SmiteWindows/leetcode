// https://leetcode.com/problems/utf-8-validation/
pub fn valid_utf8(data: Vec<i32>) -> bool {
    todo!()
}
// bit_manipulation
#[test]
#[ignore]
fn test1_393() {
    assert_eq!(valid_utf8(vec![197, 130, 1]), true);
    assert_eq!(valid_utf8(vec![235, 140, 4]), false);
}
