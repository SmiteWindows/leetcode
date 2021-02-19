// https://leetcode-cn.com/problems/verbal-arithmetic-puzzle/
pub fn is_solvable(words: Vec<String>, result: String) -> bool {
    todo!()
}
// math backtracking
#[test]
#[ignore]
fn test2_1307() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        is_solvable(vec_string!["SEND", "MORE"], "MONEY".to_string()),
        true
    );
    assert_eq!(
        is_solvable(vec_string!["SIX", "SEVEN", "SEVEN"], "TWENTY".to_string()),
        true
    );
    assert_eq!(
        is_solvable(vec_string!["THIS", "IS", "TOO"], "FUNNY".to_string()),
        true
    );
    assert_eq!(
        is_solvable(vec_string!["LEET", "CODE"], "POINT".to_string()),
        false
    );
}
