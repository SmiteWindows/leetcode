// https://leetcode.com/problems/solve-the-equation/
pub fn solve_equation(equation: String) -> String {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_640() {
    assert_eq!(
        solve_equation(String::from("x+5-3+x=6+x-2")),
        String::from("x=2")
    );
    assert_eq!(
        solve_equation(String::from("x=x")),
        String::from("Infinite solutions")
    );
    assert_eq!(solve_equation(String::from("2x=x")), String::from("x=0"));
    assert_eq!(
        solve_equation(String::from("2x+3x-6x=x+2")),
        String::from("x=-1")
    );
    assert_eq!(
        solve_equation(String::from("x=x+2")),
        String::from("No solution")
    );
}
