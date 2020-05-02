// https://leetcode.com/problems/evaluate-division/
pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    todo!()
}
// graph union_find
#[test]
#[ignore]
fn test1_399() {
    assert_eq!(
        calc_equation(
            vec![
                vec![String::from("a"), String::from("b")],
                vec![String::from("b"), String::from("c")]
            ],
            vec![2.0, 3.0],
            vec![
                vec![String::from("a"), String::from("c")],
                vec![String::from("b"), String::from("a")],
                vec![String::from("a"), String::from("e")],
                vec![String::from("a"), String::from("a")],
                vec![String::from("x"), String::from("x")]
            ]
        ),
        vec![6.0, 0.5, -1.0, 1.0, -1.0]
    );
}
