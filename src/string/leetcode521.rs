// https://leetcode.com/problems/longest-uncommon-subsequence-i/
pub fn find_lu_slength(a: String, b: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_521() {
    assert_eq!(find_lu_slength(String::from("aba"), String::from("cdc")), 3);
    assert_eq!(find_lu_slength(String::from("aaa"), String::from("bbb")), 3);
    assert_eq!(
        find_lu_slength(String::from("aaa"), String::from("aaa")),
        -1
    );
}
