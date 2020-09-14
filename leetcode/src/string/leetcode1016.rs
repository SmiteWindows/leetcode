// https://leetcode-cn.com/problems/binary-string-with-substrings-representing-1-to-n/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn query_string(s: String, n: i32) -> bool {
    for i in (1..=n).rev() {
        let t = format!("{:b}", i);
        if !s.contains(&t) {
            return false;
        }
    }
    true
}
// string
#[test]
fn test1_1016() {
    assert_eq!(query_string("0110"), 3), true);
    assert_eq!(query_string("0110"), 4), false);
}
