// https://leetcode-cn.com/problems/remove-comments/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut in_block = false;
    let mut line = "".to_string();
    let mut res = Vec::new();
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
    use leetcode_prelude::vec_string;
    assert_eq!(
        remove_comments(vec_string![
            "/*Test program */",
            "int main()",
            "{ ",
            "  // variable declaration ",
            "int a, b, c;",
            "/* This is a test",
            "   multiline  ",
            "   comment for ",
            "   testing */",
            "a = b + c;",
            "}"
        ]),
        vec_string!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]
    );
    assert_eq!(
        remove_comments(vec_string!["a/*comment", "line", "more_comment*/b"]),
        vec_string!["ab"]
    );
}
