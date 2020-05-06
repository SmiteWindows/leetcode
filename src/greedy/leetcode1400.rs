// https://leetcode.com/problems/construct-k-palindrome-strings/
pub fn can_construct(s: String, k: i32) -> bool {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1400() {
    assert_eq!(can_construct(String::from("annabelle"), 2), true);
    assert_eq!(can_construct(String::from("leetcode"), 3), false);
    assert_eq!(can_construct(String::from("true"), 4), true);
    assert_eq!(can_construct(String::from("yzyzyzyzyzyzyzy"), 2), true);
    assert_eq!(can_construct(String::from("cr"), 7), false);
}
