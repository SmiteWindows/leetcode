// https://leetcode.com/problems/longest-happy-string/
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    todo!()
}
// greedy dynamic_programming
#[test]
#[ignore]
fn test1_1405() {
    assert_eq!(longest_diverse_string(1, 1, 7), String::from("ccaccbcc"));
    assert_eq!(longest_diverse_string(2, 2, 1), String::from("aabbc"));
    assert_eq!(longest_diverse_string(7, 1, 0), String::from("aabaa"));
}
