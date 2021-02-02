// https://leetcode-cn.com/problems/simplify-path/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn simplify_path(path: String) -> String {
    let mut stack = Vec::new();
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
    assert_eq!(simplify_path("/home/".to_string()), "/home".to_string());
    assert_eq!(simplify_path("/../".to_string()), "/".to_string());
    assert_eq!(
        simplify_path("/home//foo/".to_string()),
        "/home/foo".to_string()
    );
    assert_eq!(
        simplify_path("/a/./b/../../c/".to_string()),
        "/c".to_string()
    );
    assert_eq!(
        simplify_path("/a/../../b/../c//.//".to_string()),
        "/c".to_string()
    );
    assert_eq!(
        simplify_path("/a//b////c/d//././/..".to_string()),
        "/a/b/c".to_string()
    );
}
