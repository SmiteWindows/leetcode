// https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::iter::repeat;
pub fn generate_the_string(n: i32) -> String {
    let mut s = repeat('a').take((n - 1) as usize).collect::<String>();
    s.push(if n % 2 == 0 { 'b' } else { 'a' });
    s
}
// string
#[test]
fn test1_1374() {
    assert_eq!(generate_the_string(4), String::from("aaab"));
    // assert_eq!(generate_the_string(4), String::from("pppz"));
    assert_eq!(generate_the_string(2), String::from("ab"));
    // assert_eq!(generate_the_string(2), String::from("xy"));
    assert_eq!(generate_the_string(7), String::from("aaaaaaa"));
    // assert_eq!(generate_the_string(7), String::from("holasss"));
}
