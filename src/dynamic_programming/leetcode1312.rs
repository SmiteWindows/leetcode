// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
pub fn min_insertions(s: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1312() {
    assert_eq!(min_insertions(String::from("zzazz")), 0);
    assert_eq!(min_insertions(String::from("mbadm")), 2);
    assert_eq!(min_insertions(String::from("leetcode")), 5);
    assert_eq!(min_insertions(String::from("g")), 0);
    assert_eq!(min_insertions(String::from("no")), 1);
}
