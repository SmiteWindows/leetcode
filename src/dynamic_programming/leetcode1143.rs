// https://leetcode.com/problems/longest-common-subsequence/
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1143() {
    assert_eq!(
        longest_common_subsequence(String::from("abcde"), String::from("ace")),
        3
    );
    assert_eq!(
        longest_common_subsequence(String::from("abc"), String::from("abc")),
        3
    );
    assert_eq!(
        longest_common_subsequence(String::from("abc"), String::from("def")),
        0
    );
}
