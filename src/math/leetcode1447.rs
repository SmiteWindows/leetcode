// https://leetcode.com/problems/simplified-fractions/
pub fn simplified_fractions(n: i32) -> Vec<String> {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_1447() {
    assert_eq!(simplified_fractions(2), vec![String::from("1/2")]);
    assert_eq!(
        simplified_fractions(3),
        vec![
            String::from("1/2"),
            String::from("1/3"),
            String::from("2/3")
        ]
    );
    assert_eq!(
        simplified_fractions(4),
        vec![
            String::from("1/2"),
            String::from("1/3"),
            String::from("1/4"),
            String::from("2/3"),
            String::from("3/4")
        ]
    );
    assert_eq!(simplified_fractions(1), vec![String::new()]);
}
