// https://leetcode.com/problems/remove-comments/
pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_722() {
    assert_eq!(
        remove_comments(vec![
            String::from("/*Test program */"),
            String::from("int main()"),
            String::from("{ "),
            String::from("  // variable declaration "),
            String::from("int a, b, c;"),
            String::from("/* This is a test"),
            String::from("   multiline  "),
            String::from("   comment for "),
            String::from("   testing */"),
            String::from("a = b + c;"),
            String::from("}")
        ]),
        vec![
            String::from("int main()"),
            String::from("{ "),
            String::from("  "),
            String::from("int a, b, c;"),
            String::from("a = b + c;"),
            String::from("}")
        ]
    );
    assert_eq!(
        remove_comments(vec![
            String::from("a/*comment"),
            String::from("line"),
            String::from("more_comment*/b")
        ]),
        vec![String::from("ab")]
    );
}
