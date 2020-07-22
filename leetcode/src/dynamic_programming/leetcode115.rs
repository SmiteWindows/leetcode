// https://leetcode.com/problems/distinct-subsequences/
pub fn num_distinct(s: String, t: String) -> i32 {
    todo!()
}
// dynamic_programming string
#[test]
#[ignore]
fn test1_115() {
    assert_eq!(
        num_distinct(String::from("rabbbit"), String::from("rabbit")),
        3
    );
    assert_eq!(
        num_distinct(String::from("babgbag"), String::from("bag")),
        5
    );
}
