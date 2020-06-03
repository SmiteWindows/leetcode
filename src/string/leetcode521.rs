// https://leetcode.com/problems/longest-uncommon-subsequence-i/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_lu_slength(a: String, b: String) -> i32 {
    if a == b {
        -1
    } else {
        a.len().max(b.len()) as i32
    }
}
// string
#[test]
fn test1_521() {
    assert_eq!(find_lu_slength(String::from("aba"), String::from("cdc")), 3);
    assert_eq!(find_lu_slength(String::from("aaa"), String::from("bbb")), 3);
    assert_eq!(
        find_lu_slength(String::from("aaa"), String::from("aaa")),
        -1
    );
}
