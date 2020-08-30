// https://leetcode-cn.com/problems/remove-comments/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut in_block = false;
    let mut line = "".to_string();
    let mut res = vec![];
    for s in source {
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if in_block {
                if c == '*' {
                    if let Some(&'/') = it.peek() {
                        it.next();
                        in_block = false;
                    }
                }
            } else {
                if c == '/' {
                    match it.peek() {
                        Some(&'/') => {
                            break;
                        }
                        Some(&'*') => {
                            it.next();
                            in_block = true;
                            continue;
                        }
                        _ => {}
                    }
                }
                line.push(c);
            }
        }
        if !in_block && !line.is_empty() {
            res.push(line);
            line = "".to_string();
        }
    }
    res
}
// string
#[test]
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
