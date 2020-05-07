// https://leetcode.com/problems/ambiguous-coordinates/
pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_816() {
    assert_eq!(
        ambiguous_coordinates(String::from("(123)")),
        vec![
            String::from("(1, 23)"),
            String::from("(12, 3)"),
            String::from("(1.2, 3)"),
            String::from("(1, 2.3)"),
        ]
    );
    assert_eq!(
        ambiguous_coordinates(String::from("(00011)")),
        vec![String::from("(0.001, 1)"), String::from("(0, 0.011)")]
    );
    assert_eq!(
        ambiguous_coordinates(String::from("(0123)")),
        vec![
            String::from("(0, 123)"),
            String::from("(0, 12.3)"),
            String::from("(0, 1.23)"),
            String::from("(0.1, 23)"),
            String::from("(0.1, 2.3)"),
            String::from("(0.12, 3)")
        ]
    );
    assert_eq!(
        ambiguous_coordinates(String::from("(100)")),
        vec![String::from("(10, 0)")]
    );
}
