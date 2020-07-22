// https://leetcode.com/problems/evaluate-reverse-polish-notation/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
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
    assert_eq!(
        eval_rpn(vec![
            String::from("2"),
            String::from("1"),
            String::from("+"),
            String::from("3"),
            String::from("*")
        ]),
        9
    );
    assert_eq!(
        eval_rpn(vec![
            String::from("4"),
            String::from("13"),
            String::from("5"),
            String::from("/"),
            String::from("+")
        ]),
        6
    );
    assert_eq!(
        eval_rpn(vec![
            String::from("10"),
            String::from("6"),
            String::from("9"),
            String::from("3"),
            String::from("+"),
            String::from("-11"),
            String::from("*"),
            String::from("/"),
            String::from("*"),
            String::from("17"),
            String::from("+"),
            String::from("5"),
            String::from("+"),
        ]),
        22
    );
}
