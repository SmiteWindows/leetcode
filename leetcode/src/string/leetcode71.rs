// https://leetcode.com/problems/simplify-path/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn simplify_path(path: String) -> String {
    let mut stack = vec![];
    let mut res = "".to_string();
    for s in path.split_terminator('/') {
        match s {
            ".." => {
                stack.pop();
            }
            "" | "." => {
                continue;
            }
            _ => {
                stack.push(s);
            }
        }
    }
    for s in stack {
        res += "/";
        res += s;
    }
    if res.is_empty() {
        res += "/";
    }
    res
}
// stack string
#[test]
fn test2_71() {
    assert_eq!(simplify_path(String::from("/home/")), String::from("/home"));
    assert_eq!(simplify_path(String::from("/../")), String::from("/"));
    assert_eq!(
        simplify_path(String::from("/home//foo/")),
        String::from("/home/foo")
    );
    assert_eq!(
        simplify_path(String::from("/a/./b/../../c/")),
        String::from("/c")
    );
    assert_eq!(
        simplify_path(String::from("/a/../../b/../c//.//")),
        String::from("/c")
    );
    assert_eq!(
        simplify_path(String::from("/a//b////c/d//././/..")),
        String::from("/a/b/c")
    );
}
