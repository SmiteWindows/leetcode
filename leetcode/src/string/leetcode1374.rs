// https://leetcode-cn.com/problems/generate-a-string-with-characters-that-have-odd-counts/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn generate_the_string(n: i32) -> String {
    let mut s = "a".repeat((n - 1) as usize);
    s.push(if n % 2 == 0 { 'b' } else { 'a' });
    s
}
// string
#[test]
fn test1_1374() {
    assert_eq!(generate_the_string(4), "aaab".to_string());
    // assert_eq!(generate_the_string(4), "pppz"));
    assert_eq!(generate_the_string(2), "ab".to_string());
    // assert_eq!(generate_the_string(2), "xy"));
    assert_eq!(generate_the_string(7), "aaaaaaa".to_string());
    // assert_eq!(generate_the_string(7), "holasss"));
}
