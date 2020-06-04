// https://leetcode.com/problems/license-key-formatting/
pub fn license_key_formatting(s: String, k: i32) -> String {
    todo!()
}
#[test]
#[ignore]
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
