// https://leetcode.com/problems/split-a-string-in-balanced-strings/
pub fn balanced_string_split(s: String) -> i32 {
    todo!()
}
// greedy string
#[test]
#[ignore]
fn test1_1221() {
    assert_eq!(balanced_string_split(String::from("RLRRLLRLRL")), 4);
    assert_eq!(balanced_string_split(String::from("RLLLLRRRLR")), 3);
    assert_eq!(balanced_string_split(String::from("LLLLRRRR")), 1);
    assert_eq!(balanced_string_split(String::from("RLRRRLLRLL")), 2);
}
