// https://leetcode.com/problems/longest-duplicate-substring/
pub fn longest_dup_substring(s: String) -> String {
    todo!()
}
// binary_search hash_table
#[test]
#[ignore]
fn test2_1044() {
    assert_eq!(
        longest_dup_substring(String::from("banana")),
        String::from("ana")
    );
    assert_eq!(
        longest_dup_substring(String::from("abcd")),
        String::from("")
    );
}
