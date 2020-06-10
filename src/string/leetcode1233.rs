// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
// Runtime: 56 ms
// Memory Usage: 8 MB
use std::collections::HashSet;
pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut hs = HashSet::new();
    for s in folder {
        hs.insert(s);
    }
    let mut res = vec![];
    for s in &hs {
        if !is_subfolder(&s, &hs) {
            res.push(s.to_string());
        }
    }
    res
}

fn is_subfolder(s: &str, hs: &HashSet<String>) -> bool {
    let n = s.len();
    for i in 0..n {
        if &s[i..=i] == "/" && hs.contains(&s[0..i]) {
            return true;
        }
    }
    false
}
// array string
#[test]
fn test1_1233() {
    let mut a = remove_subfolders(vec![
        String::from("/a"),
        String::from("/a/b"),
        String::from("/c/d"),
        String::from("/c/d/e"),
        String::from("/c/f"),
    ]);
    a.sort();
    assert_eq!(
        a,
        vec![
            String::from("/a"),
            String::from("/c/d"),
            String::from("/c/f")
        ]
    );
    let mut b = remove_subfolders(vec![
        String::from("/a"),
        String::from("/a/b/c"),
        String::from("/a/b/d"),
    ]);
    b.sort();
    assert_eq!(b, vec![String::from("/a")]);
    let mut c = remove_subfolders(vec![
        String::from("/a/b/c"),
        String::from("/a/b/ca"),
        String::from("/a/b/d"),
    ]);
    c.sort();
    assert_eq!(
        c,
        vec![
            String::from("/a/b/c"),
            String::from("/a/b/ca"),
            String::from("/a/b/d")
        ]
    );
}
