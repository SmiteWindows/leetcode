// https://leetcode.com/problems/unique-substrings-in-wraparound-string/
pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_467() {
    assert_eq!(find_substring_in_wrapround_string(String::from("a")), 1);
    assert_eq!(find_substring_in_wrapround_string(String::from("cac")), 2);
    assert_eq!(find_substring_in_wrapround_string(String::from("zab")), 6);
}
