//https://leetcode.com/problems/simplify-path/
pub fn simplify_path(path: String) -> String {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test1_71() {
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
