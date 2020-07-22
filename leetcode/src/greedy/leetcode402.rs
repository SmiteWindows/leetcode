// https://leetcode.com/problems/remove-k-digits/
pub fn remove_kdigits(num: String, k: i32) -> String {
    todo!()
}
// stack greedy
#[test]
#[ignore]
fn test2_402() {
    assert_eq!(
        remove_kdigits(String::from("1432219"), 3),
        String::from("1219")
    );
    assert_eq!(
        remove_kdigits(String::from("10200"), 1),
        String::from("200")
    );
    assert_eq!(remove_kdigits(String::from("10"), 2), String::from("0"));
}
