// https://leetcode-cn.com/problems/decode-ways-ii/
pub fn num_decodings(s: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_639() {
    assert_eq!(num_decodings("*".to_string()), 9);
    assert_eq!(num_decodings("1*".to_string()), 18);
}
