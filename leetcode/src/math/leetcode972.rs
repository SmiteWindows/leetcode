// https://leetcode-cn.com/problems/equal-rational-numbers/
pub fn is_rational_equal(s: String, t: String) -> bool {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_972() {
    assert_eq!(
        is_rational_equal(String::from("0.(52)"), String::from("0.5(25)")),
        true
    );
    assert_eq!(
        is_rational_equal(String::from("0.1666(6)"), String::from("0.166(66)")),
        true
    );
    assert_eq!(
        is_rational_equal(String::from("0.9(9)"), String::from("1.")),
        true
    );
}
