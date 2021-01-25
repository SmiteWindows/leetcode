// https://leetcode-cn.com/problems/longest-uncommon-subsequence-i/
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
    assert_eq!(find_lu_slength("aba".to_string(), "cdc".to_string()), 3);
    assert_eq!(find_lu_slength("aaa".to_string(), "bbb".to_string()), 3);
    assert_eq!(find_lu_slength("aaa".to_string(), "aaa".to_string()), -1);
}
