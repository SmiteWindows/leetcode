// https://leetcode-cn.com/problems/count-different-palindromic-subsequences/
pub fn count_palindromic_subsequences(s: String) -> i32 {
    todo!()
}
// dynamic_programming string
#[test]
#[ignore]
fn test2_730() {
    assert_eq!(count_palindromic_subsequences("bccb".to_string()), 6);
    assert_eq!(
        count_palindromic_subsequences(
            "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()
        ),
        104860361
    );
}
