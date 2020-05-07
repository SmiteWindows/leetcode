// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    todo!()
}
// array string
#[test]
#[ignore]
fn test1_1233() {
    assert_eq!(
        remove_subfolders(vec![
            String::from("/a"),
            String::from("/a/b"),
            String::from("/c/d"),
            String::from("/c/d/e"),
            String::from("/c/f")
        ]),
        vec![
            String::from("/a"),
            String::from("/c/d"),
            String::from("/c/f")
        ]
    );
    assert_eq!(
        remove_subfolders(vec![
            String::from("/a"),
            String::from("/a/b/c"),
            String::from("/a/b/d")
        ]),
        vec![String::from("/a")]
    );
    assert_eq!(
        remove_subfolders(vec![
            String::from("/a/b/c"),
            String::from("/a/b/ca"),
            String::from("/a/b/d")
        ]),
        vec![
            String::from("/a/b/c"),
            String::from("/a/b/ca"),
            String::from("/a/b/d")
        ]
    );
}
