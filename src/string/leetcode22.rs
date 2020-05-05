// https://leetcode.com/problems/generate-parentheses/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    todo!()
}
// string backtracking
#[test]
#[ignore]
fn test2_22() {
    assert_eq!(
        generate_parenthesis(3),
        vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()")
        ]
    );
}
