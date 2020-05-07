// https://leetcode.com/problems/swap-for-longest-repeated-character-substring/
pub fn max_rep_opt1(text: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1156() {
    assert_eq!(max_rep_opt1(String::from("ababa")), 3);
    assert_eq!(max_rep_opt1(String::from("aaabaaa")), 6);
    assert_eq!(max_rep_opt1(String::from("aaabbaaa")), 4);
    assert_eq!(max_rep_opt1(String::from("aaaaa")), 5);
    assert_eq!(max_rep_opt1(String::from("abcdef")), 1);
}
