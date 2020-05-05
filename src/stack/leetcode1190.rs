// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
pub fn reverse_parentheses(s: String) -> String {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_1190() {
    assert_eq!(
        reverse_parentheses(String::from("(abcd)")),
        String::from("dcba")
    );
    assert_eq!(
        reverse_parentheses(String::from("(u(love)i)")),
        String::from("iloveu")
    );
    assert_eq!(
        reverse_parentheses(String::from("(ed(et(oc))el)")),
        String::from("leetcode")
    );
    assert_eq!(
        reverse_parentheses(String::from("a(bcdefghijkl(mno)p)q")),
        String::from("apmnolkjihgfedcbq")
    );
}
