// https://leetcode.com/problems/generate-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut cur: String = "".to_string();
    dfs(n, n, &mut cur, &mut res);
    res
}

fn dfs(left: i32, right: i32, cur: &mut String, all: &mut Vec<String>) {
    if left == 0 && right == 0 {
        all.push(cur.to_string());
    } else {
        if left > 0 {
            cur.push('(');
            dfs(left - 1, right, cur, all);
            cur.pop();
        }
        if right > left {
            cur.push(')');
            dfs(left, right - 1, cur, all);
            cur.pop();
        }
    }
}
// string backtracking
#[test]
fn test2_22() {
    assert_eq!(
        generate_parenthesis(3),
        vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()")
        ]
    );
}
