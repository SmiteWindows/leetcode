// https://leetcode.com/problems/string-to-integer-atoi/
pub fn my_atoi(str: String) -> i32 {
    todo!()
}
// math string
#[test]
#[ignore]
fn test1_8() {
    assert_eq!(my_atoi(String::from("42")), 42);
    assert_eq!(my_atoi(String::from("   -42")), -42);
    assert_eq!(my_atoi(String::from("4193 with words")), 4193);
    assert_eq!(my_atoi(String::from("words and 987")), 0);
    assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
}
