// https://leetcode.com/problems/greatest-common-divisor-of-strings/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let n1 = str1.len();
    let n2 = str2.len();
    let mut n = usize::min(n1, n2);
    while n > 0 {
        if n1 % n != 0 || n2 % n != 0 {
            n -= 1;
            continue;
        }
        let s1 = &str1[0..n];
        let s2 = &str2[0..n];
        if s1 != s2 {
            n -= 1;
            continue;
        }
        let k1 = n1 / n;
        let k2 = n2 / n;
        if str1.matches(s1).count() == k1 && str2.matches(s2).count() == k2 {
            return s1.to_string();
        }
        n -= 1;
    }
    "".to_string()
}
// string
#[test]
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
