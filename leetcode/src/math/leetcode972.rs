// https://leetcode-cn.com/problems/equal-rational-numbers/
pub fn is_rational_equal(s: String, t: String) -> bool {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_972() {
    assert_eq!(
        is_rational_equal("0.(52)".to_string(), "0.5(25)".to_string()),
        true
    );
    assert_eq!(
        is_rational_equal("0.1666(6)".to_string(), "0.166(66)".to_string()),
        true
    );
    assert_eq!(
        is_rational_equal("0.9(9)".to_string(), "1.".to_string()),
        true
    );
}
