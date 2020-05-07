// https://leetcode.com/problems/greatest-common-divisor-of-strings/
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1071() {
    assert_eq!(
        gcd_of_strings(String::from("ABCABC"), String::from("ABC")),
        String::from("ABC")
    );
    assert_eq!(
        gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
        String::from("AB")
    );
    assert_eq!(
        gcd_of_strings(String::from("LEET"), String::from("CODE")),
        String::from("")
    );
}
