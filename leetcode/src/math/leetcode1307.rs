// https://leetcode-cn.com/problems/verbal-arithmetic-puzzle/
pub fn is_solvable(words: Vec<String>, result: String) -> bool {
    todo!()
}
// math backtracking
#[test]
#[ignore]
fn test1_1307() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        is_solvable(vec_string!["SEND", "MORE"], String::from("MONEY")),
        true
    );
    assert_eq!(
        is_solvable(vec_string!["SIX", "SEVEN", "SEVEN"], String::from("TWENTY")),
        true
    );
    assert_eq!(
        is_solvable(vec_string!["THIS", "IS", "TOO"], String::from("FUNNY")),
        true
    );
    assert_eq!(
        is_solvable(vec_string!["LEET", "CODE"], String::from("POINT")),
        false
    );
}
