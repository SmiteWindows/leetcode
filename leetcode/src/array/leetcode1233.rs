// https://leetcode-cn.com/problems/remove-sub-folders-from-the-filesystem/
// Runtime: 56 ms
// Memory Usage: 8 MB
use std::collections::HashSet;
pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut hs = HashSet::new();
    for s in folder {
        hs.insert(s);
    }
    let mut res = Vec::new();
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
fn test2_1233() {
    use leetcode_prelude::{assert_eq_sorted, vec_string};
    assert_eq_sorted!(
        remove_subfolders(vec_string!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]),
        vec_string!["/a", "/c/d", "/c/f"]
    );
    assert_eq_sorted!(
        remove_subfolders(vec_string!["/a", "/a/b/c", "/a/b/d"]),
        vec_string!["/a"]
    );
    assert_eq_sorted!(
        remove_subfolders(vec_string!["/a/b/c", "/a/b/ca", "/a/b/d"]),
        vec_string!["/a/b/c", "/a/b/ca", "/a/b/d"]
    );
}
