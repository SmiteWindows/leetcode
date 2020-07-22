// https://leetcode.com/problems/count-different-palindromic-subsequences/
pub fn count_palindromic_subsequences(s: String) -> i32 {
    todo!()
}
// dynamic_programming string
#[test]
#[ignore]
fn test1_730() {
    assert_eq!(count_palindromic_subsequences(String::from("bccb")), 6);
    assert_eq!(
        count_palindromic_subsequences(String::from(
            "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
        )),
        104860361
    );
}
