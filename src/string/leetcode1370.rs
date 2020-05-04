// https://leetcode.com/problems/increasing-decreasing-string/
pub fn sort_string(s: String) -> String {
    todo!()
}
// sort string
#[test]
#[ignore]
fn test2_1370() {
    assert_eq!(
        sort_string(String::from("aaaabbbbcccc")),
        String::from("abccbaabccba")
    );
    assert_eq!(sort_string(String::from("rat")), String::from("art"));
    assert_eq!(
        sort_string(String::from("leetcode")),
        String::from("cdelotee")
    );
    assert_eq!(
        sort_string(String::from("ggggggg")),
        String::from("ggggggg")
    );
    assert_eq!(sort_string(String::from("spo")), String::from("ops"));
}
