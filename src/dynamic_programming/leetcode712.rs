// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_712() {
    assert_eq!(
        minimum_delete_sum(String::from("sea"), String::from("eat")),
        231
    );
    assert_eq!(
        minimum_delete_sum(String::from("delete"), String::from("leet")),
        403
    );
}
