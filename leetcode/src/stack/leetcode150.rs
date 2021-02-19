// https://leetcode-cn.com/problems/evaluate-reverse-polish-notation/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for tok in tokens {
        match tok.as_ref() {
            "+" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            }
            "-" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
            }
            "*" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            "/" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left / right);
            }
            _ => {
                stack.push(tok.parse::<i32>().unwrap());
            }
        }
    }
    stack[0]
}
// stack
#[test]
fn test1_150() {
    use leetcode_prelude::vec_string;
    assert_eq!(eval_rpn(vec_string!["2", "1", "+", "3", "*"]), 9);
    assert_eq!(eval_rpn(vec_string!["4", "13", "5", "/", "+"]), 6);
    assert_eq!(
        eval_rpn(vec_string![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ]),
        22
    );
}
