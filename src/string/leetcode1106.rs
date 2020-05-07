// https://leetcode.com/problems/parsing-a-boolean-expression/
pub fn parse_bool_expr(expression: String) -> bool {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1106() {
    assert_eq!(parse_bool_expr(String::from("!(f)")), true);
    assert_eq!(parse_bool_expr(String::from("|(f,t)")), true);
    assert_eq!(parse_bool_expr(String::from("&(t,f)")), false);
    assert_eq!(parse_bool_expr(String::from("|(&(t,f,t),!(t))")), false);
}
