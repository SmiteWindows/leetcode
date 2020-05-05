// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/
pub fn remove_duplicates(s: String, k: i32) -> String {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_1209() {
    assert_eq!(
        remove_duplicates(String::from("abcd"), 2),
        String::from("abcd")
    );
    assert_eq!(
        remove_duplicates(String::from("deeedbbcccbdaa"), 3),
        String::from("aa")
    );
    assert_eq!(
        remove_duplicates(String::from("pbbcggttciiippooaais"), 2),
        String::from("ps")
    );
}
