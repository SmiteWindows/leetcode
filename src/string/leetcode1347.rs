// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
pub fn min_steps(s: String, t: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1347() {
    assert_eq!(min_steps(String::from("bab"), String::from("aba")), 1);
    assert_eq!(
        min_steps(String::from("leetcode"), String::from("practice")),
        5
    );
    assert_eq!(
        min_steps(String::from("anagram"), String::from("mangaar")),
        0
    );
    assert_eq!(min_steps(String::from("xxyyzz"), String::from("xxyyzz")), 0);
    assert_eq!(min_steps(String::from("friend"), String::from("family")), 4);
}
