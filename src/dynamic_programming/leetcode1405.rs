// https://leetcode.com/problems/longest-happy-string/
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    todo!()
}
// dynamic_programming greedy
#[test]
#[ignore]
fn test2_1405() {
    assert_eq!(longest_diverse_string(1, 1, 7), String::from("ccaccbcc"));
    assert_eq!(longest_diverse_string(2, 2, 1), String::from("aabbc"));
    assert_eq!(longest_diverse_string(7, 1, 0), String::from("aabaa"));
}
