// https://leetcode.com/problems/get-equal-substrings-within-budget/
pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    todo!()
}
// sliding_window array
#[test]
#[ignore]
fn test2_1208() {
    assert_eq!(
        equal_substring(String::from("abcd"), String::from("bcdf"), 3),
        3
    );
    assert_eq!(
        equal_substring(String::from("abcd"), String::from("cdef"), 3),
        1
    );
    assert_eq!(
        equal_substring(String::from("abcd"), String::from("acde"), 0),
        1
    );
}
