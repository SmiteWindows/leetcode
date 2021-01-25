// https://leetcode-cn.com/problems/longest-chunked-palindrome-decomposition/
pub fn longest_decomposition(text: String) -> i32 {
    todo!()
}
// dynamic_programming rolling_hash
#[test]
#[ignore]
fn test2_1147() {
    assert_eq!(
        longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string()),
        7
    );
    assert_eq!(longest_decomposition("merchant".to_string()), 1);
    assert_eq!(
        longest_decomposition("antaprezatepzapreanta".to_string()),
        11
    );
    assert_eq!(longest_decomposition("aaa".to_string()), 3);
}
