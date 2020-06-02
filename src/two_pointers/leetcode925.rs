// https://leetcode.com/problems/long-pressed-name/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let n = name.len();
    let m = typed.len();
    let mut i = 0;
    let mut j = 0;
    let s = name.chars().collect::<Vec<_>>();
    let t = typed.chars().collect::<Vec<_>>();
    while i < n && j < m {
        if s[i] == t[j] {
            let mut a = 0;
            while i + 1 < n && s[i + 1] == s[i] {
                i += 1;
                a += 1;
            }
            i += 1;
            let mut b = 0;
            while j + 1 < m && t[j + 1] == t[j] {
                j += 1;
                b += 1;
            }
            j += 1;
            if a > b {
                return false;
            }
        } else {
            return false;
        }
    }
    i == n && j == m
}
// two_pointers string
#[test]
fn test1_925() {
    assert_eq!(
        is_long_pressed_name(String::from("alex"), String::from("aaleex")),
        true
    );
    assert_eq!(
        is_long_pressed_name(String::from("saeed"), String::from("ssaaedd")),
        false
    );
    assert_eq!(
        is_long_pressed_name(String::from("leelee"), String::from("lleeelee")),
        true
    );
    assert_eq!(
        is_long_pressed_name(String::from("laiden"), String::from("laiden")),
        true
    );
}
