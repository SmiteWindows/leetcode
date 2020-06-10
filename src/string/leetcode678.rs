// https://leetcode.com/problems/valid-parenthesis-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn check_valid_string(s: String) -> bool {
    let mut lo = 0;
    let mut hi = 0;
    for c in s.chars() {
        match c {
            '(' => {
                lo += 1;
                hi += 1;
            }
            ')' => {
                lo = 0.max(lo - 1);
                hi -= 1;
            }
            _ => {
                lo = 0.max(lo - 1);
                hi += 1;
            }
        }
        if hi < 0 {
            return false;
        }
    }
    lo == 0
}
// string
#[test]
fn test1_678() {
    assert_eq!(check_valid_string(String::from("()")), true);
    assert_eq!(check_valid_string(String::from("(*)")), true);
    assert_eq!(check_valid_string(String::from("(*))")), true);
}
