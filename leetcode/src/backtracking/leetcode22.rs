// https://leetcode.com/problems/generate-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut v = vec![];
    gen(&mut res, &mut v, n, n);
    res
}

fn gen(res: &mut Vec<String>, v: &mut Vec<char>, left: i32, right: i32) {
    if left == 0 && right == 0 {
        res.push(v.iter().collect::<String>());
    } else {
        if left > 0 {
            v.push('(');
            gen(res, v, left - 1, right);
            v.pop();
        }
        if right > left {
            v.push(')');
            gen(res, v, left, right - 1);
            v.pop();
        }
    }
}
// string backtracking
#[test]
fn test1_22() {
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
