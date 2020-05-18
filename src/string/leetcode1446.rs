// https://leetcode.com/problems/consecutive-characters/
pub fn max_power(s: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1446() {
    assert_eq!(max_power(String::from("leetcode")), 2);
    assert_eq!(max_power(String::from("abbcccddddeeeeedcba")), 5);
    assert_eq!(max_power(String::from("triplepillooooow")), 5);
    assert_eq!(max_power(String::from("hooraaaaaaaaaaay")), 11);
    assert_eq!(max_power(String::from("tourist")), 1);
}
