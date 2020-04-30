// https://leetcode.com/problems/basic-calculator/
pub fn calculate(s: String) -> i32 {
    todo!()
}
// math stack
#[test]
#[ignore]
fn test1_224() {
    assert_eq!(calculate(String::from("1 + 1")), 2);
    assert_eq!(calculate(String::from(" 2-1 + 2 ")), 3);
    assert_eq!(calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
}
