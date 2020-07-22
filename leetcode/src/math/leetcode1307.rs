// https://leetcode.com/problems/verbal-arithmetic-puzzle/
pub fn is_solvable(words: Vec<String>, result: String) -> bool {
    todo!()
}
// math backtracking
#[test]
#[ignore]
fn test1_1307() {
    assert_eq!(
        is_solvable(
            vec![String::from("SEND"), String::from("MORE")],
            String::from("MONEY")
        ),
        true
    );
    assert_eq!(
        is_solvable(
            vec![
                String::from("SIX"),
                String::from("SEVEN"),
                String::from("SEVEN")
            ],
            String::from("TWENTY")
        ),
        true
    );
    assert_eq!(
        is_solvable(
            vec![
                String::from("THIS"),
                String::from("IS"),
                String::from("TOO")
            ],
            String::from("FUNNY")
        ),
        true
    );
    assert_eq!(
        is_solvable(
            vec![String::from("LEET"), String::from("CODE")],
            String::from("POINT")
        ),
        false
    );
}
