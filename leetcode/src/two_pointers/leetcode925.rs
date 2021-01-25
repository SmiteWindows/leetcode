// https://leetcode-cn.com/problems/long-pressed-name/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let nl = name.len();
    let ml = typed.len();
    let mut i = 0;
    let mut j = 0;
    let nc = name.chars().collect::<Vec<_>>();
    let tc = typed.chars().collect::<Vec<_>>();
    while i < nl && j < ml {
        if nc[i] == tc[j] {
            let mut a = 0;
            while i + 1 < nl && nc[i + 1] == nc[i] {
                i += 1;
                a += 1;
            }
            i += 1;
            let mut b = 0;
            while j + 1 < ml && tc[j + 1] == tc[j] {
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
    i == nl && j == ml
}
// two_pointers string
#[test]
fn test1_925() {
    assert_eq!(
        is_long_pressed_name("alex".to_string(), "aaleex".to_string()),
        true
    );
    assert_eq!(
        is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
        false
    );
    assert_eq!(
        is_long_pressed_name("leelee".to_string(), "lleeelee".to_string()),
        true
    );
    assert_eq!(
        is_long_pressed_name("laiden".to_string(), "laiden".to_string()),
        true
    );
}
