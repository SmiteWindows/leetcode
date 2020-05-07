// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
pub fn find_the_longest_substring(s: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1371() {
    assert_eq!(
        find_the_longest_substring(String::from("eleetminicoworoep")),
        13
    );
    assert_eq!(
        find_the_longest_substring(String::from("leetcodeisgreat")),
        5
    );
    assert_eq!(find_the_longest_substring(String::from("bcbcbc")), 6);
}
