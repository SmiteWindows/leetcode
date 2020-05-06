// https://leetcode.com/problems/find-all-anagrams-in-a-string/
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_438() {
    assert_eq!(
        find_anagrams(String::from("cbaebabacd"), String::from("abc")),
        vec![0, 6]
    );
    assert_eq!(
        find_anagrams(String::from("abab"), String::from("ab")),
        vec![0, 1, 2]
    );
}
