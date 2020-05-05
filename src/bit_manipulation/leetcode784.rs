// https://leetcode.com/problems/letter-case-permutation/
pub fn letter_case_permutation(s: String) -> Vec<String> {
    todo!()
}
// backtracking bit_manipulation
#[test]
#[ignore]
fn test2_784() {
    assert_eq!(
        letter_case_permutation(String::from("a1b2")),
        vec![
            String::from("a1b2"),
            String::from("a1B2"),
            String::from("A1b2"),
            String::from("A1B2")
        ]
    );
    assert_eq!(
        letter_case_permutation(String::from("3z4")),
        vec![String::from("3z4"), String::from("3Z4")]
    );
    assert_eq!(
        letter_case_permutation(String::from("12345")),
        vec![String::from("12345")]
    );
}