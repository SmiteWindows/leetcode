// https://leetcode.com/problems/complex-number-multiplication/
pub fn complex_number_multiply(a: String, b: String) -> String {
    todo!()
}
// math string
#[test]
#[ignore]
fn test1_537() {
    assert_eq!(
        complex_number_multiply(String::from("1+1i"), String::from("1+1i")),
        String::from("0+2i")
    );
    assert_eq!(
        complex_number_multiply(String::from("1+-1i"), String::from("1+-1i")),
        String::from("0+-2i")
    );
}
