// https://leetcode.com/problems/longest-chunked-palindrome-decomposition/
pub fn longest_decomposition(text: String) -> i32 {
    todo!()
}
// dynamic_programming rolling_hash
#[test]
#[ignore]
fn test2_1147() {
    assert_eq!(
        longest_decomposition(String::from("ghiabcdefhelloadamhelloabcdefghi")),
        7
    );
    assert_eq!(longest_decomposition(String::from("merchant")), 1);
    assert_eq!(
        longest_decomposition(String::from("antaprezatepzapreanta")),
        11
    );
    assert_eq!(longest_decomposition(String::from("aaa")), 3);
}
