// https://leetcode.com/problems/remove-invalid-parentheses/
// Runtime: 12 ms
// Memory Usage: 2.1 MB
#![allow(clippy::too_many_arguments)]
use std::collections::HashSet;
pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut cur = vec![];
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut min = usize::MAX;
    let mut res: HashSet<String> = HashSet::new();
    dfs(0, 0, 0, &mut cur, &mut res, &mut min, &s, n);
    res.into_iter().collect()
}

fn dfs(
    start: usize,
    left: usize,
    remove: usize,
    cur: &mut Vec<char>,
    all: &mut HashSet<String>,
    min: &mut usize,
    s: &[char],
    n: usize,
) {
    if start == n {
        if left != 0 {
            return;
        }
        if remove > *min {
            return;
        }
        if remove < *min {
            *min = remove;
            all.clear();
        }
        let s = cur.iter().copied().collect();
        all.insert(s);
    } else {
        match s[start] {
            '(' => {
                cur.push('(');
                dfs(start + 1, left + 1, remove, cur, all, min, s, n);
                cur.pop();
                dfs(start + 1, left, remove + 1, cur, all, min, s, n);
            }
            ')' => {
                if left > 0 {
                    cur.push(')');
                    dfs(start + 1, left - 1, remove, cur, all, min, s, n);
                    cur.pop();
                }
                dfs(start + 1, left, remove + 1, cur, all, min, s, n);
            }
            _ => {
                cur.push(s[start]);
                dfs(start + 1, left, remove, cur, all, min, s, n);
                cur.pop();
            }
        }
    }
}
// depth_first_search breadth_first_search
#[test]
fn test1_301() {
    assert_eq!(
        remove_invalid_parentheses(String::from("()())()")),
        vec![String::from("()()()"), String::from("(())()")]
    );
    assert_eq!(
        remove_invalid_parentheses(String::from("(a)())()")),
        vec![String::from("(a)()()"), String::from("(a())()")]
    );
    assert_eq!(
        remove_invalid_parentheses(String::from(")(")),
        vec![String::from("")]
    );
}
