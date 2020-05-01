// https://leetcode.com/problems/fraction-addition-and-subtraction/
pub fn fraction_addition(expression: String) -> String {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_592() {
    assert_eq!(
        fraction_addition(String::from("-1/2+1/2")),
        String::from("0/1")
    );
    assert_eq!(
        fraction_addition(String::from("-1/2+1/2+1/3")),
        String::from("1/3")
    );
    assert_eq!(
        fraction_addition(String::from("1/3-1/2")),
        String::from("-1/6")
    );
    assert_eq!(
        fraction_addition(String::from("5/3+1/3")),
        String::from("2/1")
    );
}
