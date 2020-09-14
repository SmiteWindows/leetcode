// https://leetcode-cn.com/problems/parse-lisp-expression/
pub fn evaluate(expression: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_736() {
    assert_eq!(evaluate("(add 1 2)".to_string()), 3);
    assert_eq!(evaluate("(mult 3 (add 2 3))".to_string()), 15);
    assert_eq!(evaluate("(let x 2 (mult x 5))".to_string()), 10);
    assert_eq!(
        evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string()),
        14
    );
    assert_eq!(evaluate("(let x 3 x 2 x)".to_string()), 2);
    assert_eq!(
        evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string()),
        5
    );
    assert_eq!(
        evaluate("(let x 2 (add (let x 3 (let x 4 x)) x))".to_string()),
        6
    );
    assert_eq!(evaluate("(let a1 3 b2 (add a1 1) b2)".to_string()), 4);
}
