// https://leetcode.com/problems/find-the-difference/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_the_difference(s: String, t: String) -> char {
    let mut res = 0 as u8;
    for (i, ch) in s.char_indices() {
        res ^= ch as u8;
    }
    for (i, ch) in t.char_indices() {
        res ^= ch as u8;
    }
    res as char
}
// bit_manipulation hash_table
#[test]
fn test1_389() {
    assert_eq!(
        find_the_difference(String::from("abcd"), String::from("abcde")),
        'e'
    );
}
