// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
pub fn letter_combinations(digits: String) -> Vec<String> {
    todo!()
}
// string backtracking
#[test]
#[ignore]
fn test2_17() {
    assert_eq!(
        letter_combinations(String::from("23")),
        vec![
            String::from("ad"),
            String::from("ae"),
            String::from("af"),
            String::from("bd"),
            String::from("be"),
            String::from("bf"),
            String::from("cd"),
            String::from("ce"),
            String::from("cf")
        ]
    );
}