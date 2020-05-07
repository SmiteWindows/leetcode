// https://leetcode.com/problems/basic-calculator-ii/
pub fn calculate(s: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_227() {
    assert_eq!(calculate(String::from("3+2*2")), 7);
    assert_eq!(calculate(String::from(" 3/2 ")), 1);
    assert_eq!(calculate(String::from(" 3+5 / 2 ")), 5);
}
