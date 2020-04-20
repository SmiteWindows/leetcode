// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
pub fn get_happy_string(n: i32, k: i32) -> String {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_1415() {
    assert_eq!(get_happy_string(1, 3), String::from("c"));
    assert_eq!(get_happy_string(1, 4), String::from(""));
    assert_eq!(get_happy_string(3, 9), String::from("cab"));
    assert_eq!(get_happy_string(2, 7), String::from(""));
    assert_eq!(get_happy_string(10, 100), String::from("abacbabacb"));
}
