// https://leetcode-cn.com/problems/decode-ways-ii/
pub fn num_decodings(s: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_639() {
    assert_eq!(num_decodings(String::from("*")), 9);
    assert_eq!(num_decodings(String::from("1*")), 18);
}
