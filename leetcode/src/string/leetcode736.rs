// https://leetcode-cn.com/problems/parse-lisp-expression/
pub fn evaluate(expression: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_736() {
    assert_eq!(evaluate(String::from("(add 1 2)")), 3);
    assert_eq!(evaluate(String::from("(mult 3 (add 2 3))")), 15);
    assert_eq!(evaluate(String::from("(let x 2 (mult x 5))")), 10);
    assert_eq!(
        evaluate(String::from("(let x 2 (mult x (let x 3 y 4 (add x y))))")),
        14
    );
    assert_eq!(evaluate(String::from("(let x 3 x 2 x)")), 2);
    assert_eq!(
        evaluate(String::from("(let x 1 y 2 x (add x y) (add x y))")),
        5
    );
    assert_eq!(
        evaluate(String::from("(let x 2 (add (let x 3 (let x 4 x)) x))")),
        6
    );
    assert_eq!(evaluate(String::from("(let a1 3 b2 (add a1 1) b2)")), 4);
}
