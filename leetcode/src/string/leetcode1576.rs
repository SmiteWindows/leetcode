// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn modify_string(s: String) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let n = s.len();
    for i in 0..n {
        if s[i] == '?' {
            s[i] = 'a';
            while (i > 0 && s[i] == s[i - 1]) || (i + 1 < n && s[i] == s[i + 1]) {
                s[i] = (s[i] as u8 + 1) as char;
            }
        }
    }
    s.into_iter().collect()
}
// string
#[test]
fn test1_1576() {
    assert_eq!(modify_string(String::from("?zs")), String::from("azs"));
    assert_eq!(modify_string(String::from("ubv?w")), String::from("ubvaw"));
    assert_eq!(
        modify_string(String::from("j?qg??b")),
        String::from("jaqgacb")
    );
    // assert_eq!(
    //     modify_string(String::from("??yw?ipkj?")),
    //     String::from("acywaipkja")
    // );
}
