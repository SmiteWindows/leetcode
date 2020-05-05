// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
pub fn min_remove_to_make_valid(s: String) -> String {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test2_1249() {
    assert_eq!(
        min_remove_to_make_valid(String::from("lee(t(c)o)de)")),
        String::from("lee(t(c)o)de")
    );
    assert_eq!(
        min_remove_to_make_valid(String::from("a)b(c)d")),
        String::from("ab(c)d")
    );
    assert_eq!(
        min_remove_to_make_valid(String::from("))((")),
        String::from("")
    );
    assert_eq!(
        min_remove_to_make_valid(String::from("(a(b(c)d)")),
        String::from("a(b(c)d)")
    );
}
