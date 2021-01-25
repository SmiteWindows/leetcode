// https://leetcode-cn.com/problems/remove-invalid-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
#![allow(clippy::too_many_arguments)]
pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut res: Vec<String> = vec![];

    if s.is_empty() {
        res.push(String::new());
        return res;
    }

    helper(&mut res, &s, 0, 0, ['(', ')']);

    res
}

fn helper(res: &mut Vec<String>, s: &str, last_i: usize, last_j: usize, par: [char; 2]) {
    let mut stack: i32 = 0;

    for i in last_i..s.len() {
        let ch = s.chars().nth(i).unwrap();

        if ch == par[0] {
            stack += 1;
        }
        if ch == par[1] {
            stack -= 1;
        }

        if stack >= 0 {
            continue;
        }

        for j in last_j..=i {
            if s.chars().nth(j).unwrap() == par[1]
                && (j == last_j || s.chars().nth(j - 1).unwrap() != par[1])
            {
                let mut s_vec: Vec<char> = s.chars().collect();
                s_vec.remove(j);
                let s_vec: String = s_vec.into_iter().collect();
                helper(res, &s_vec, i, j, par);
            }
        }

        return;
    }

    let mut rev_vec: Vec<char> = s.chars().collect();
    rev_vec.reverse();
    let rev_s: String = rev_vec.into_iter().collect();

    if par[0] == '(' {
        helper(res, &rev_s, 0, 0, [')', '(']);
    } else {
        res.push(rev_s);
    }
}
// depth_first_search breadth_first_search
#[test]
fn test2_301() {
    use leetcode_prelude::{assert_eq_sorted, vec_string};
    assert_eq_sorted!(
        remove_invalid_parentheses("()())()".to_string()),
        vec_string!["()()()", "(())()"]
    );
    assert_eq_sorted!(
        remove_invalid_parentheses("(a)())()".to_string()),
        vec_string!["(a)()()", "(a())()"]
    );
    assert_eq!(
        remove_invalid_parentheses(")(".to_string()),
        vec_string![""]
    );
}
