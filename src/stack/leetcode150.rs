// https://leetcode.com/problems/evaluate-reverse-polish-notation/
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    todo!()
}
// stack
#[test]
#[ignore]
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
