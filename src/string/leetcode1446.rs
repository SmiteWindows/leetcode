// https://leetcode.com/problems/consecutive-characters/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn max_power(s: String) -> i32 {
    let mut prev: (char, usize) = (' ', 0);
    let mut res = 0;
    for c in s.chars() {
        if c == prev.0 {
            prev.1 += 1;
        } else {
            prev = (c, 1);
        }
        res = res.max(prev.1);
    }
    res as i32
}
// string
#[test]
fn test1_1446() {
    assert_eq!(max_power(String::from("leetcode")), 2);
    assert_eq!(max_power(String::from("abbcccddddeeeeedcba")), 5);
    assert_eq!(max_power(String::from("triplepillooooow")), 5);
    assert_eq!(max_power(String::from("hooraaaaaaaaaaay")), 11);
    assert_eq!(max_power(String::from("tourist")), 1);
}
