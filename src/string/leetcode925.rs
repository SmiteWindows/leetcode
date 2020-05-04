// https://leetcode.com/problems/long-pressed-name/
pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    todo!()
}
// two_pointers string
#[test]
#[ignore]
fn test2_925() {
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
