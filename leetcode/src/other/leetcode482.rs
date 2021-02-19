// https://leetcode-cn.com/problems/license-key-formatting/
// Runtime: 0 ms
// Memory Usage: 2.5 MB
pub fn license_key_formatting(s: String, k: i32) -> String {
    let mut res = Vec::new();
    let mut i = 0;
    for c in s.chars().rev() {
        if c != '-' {
            res.push(c);
            i += 1;
            if i == k {
                i = 0;
                res.push('-');
            }
        }
    }
    if let Some(&'-') = res.last() {
        res.pop();
    }
    res.iter().rev().collect::<String>().to_ascii_uppercase()
}
#[test]
fn test482() {
    assert_eq!(
        license_key_formatting("5F3Z-2e-9-w".to_string(), 4),
        "5F3Z-2E9W".to_string()
    );
    assert_eq!(
        license_key_formatting("2-5g-3-J".to_string(), 2),
        "2-5G-3J".to_string()
    );
}
