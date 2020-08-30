// https://leetcode-cn.com/problems/longest-uncommon-subsequence-ii/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::cmp::Reverse;
pub fn find_lu_slength(strs: Vec<String>) -> i32 {
    let mut strs = strs;
    let n = strs.len();
    strs.sort_by_key(|s| Reverse(s.len()));
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if i != j && !is_subsequence_of(&strs[i], &strs[j]) {
                count += 1;
            }
        }
        if count == n - 1 {
            return strs[i].len() as i32;
        }
    }
    -1
}

fn is_subsequence_of(c: &str, s: &str) -> bool {
    let mut it = c.chars().peekable();
    for c in s.chars() {
        if let Some(&first) = it.peek() {
            if first == c {
                it.next();
            }
        }
    }
    it.next().is_none()
}
// string
#[test]
fn test1_522() {
    assert_eq!(
        find_lu_slength(vec![
            String::from("aba"),
            String::from("cdc"),
            String::from("eae")
        ]),
        3
    );
}
