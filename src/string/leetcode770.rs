// https://leetcode.com/problems/basic-calculator-iv/
pub fn basic_calculator_iv(
    expression: String,
    evalvars: Vec<String>,
    evalints: Vec<i32>,
) -> Vec<String> {
    todo!()
}
// stack hash_table string
#[test]
#[ignore]
fn test2_770() {
    assert_eq!(
        basic_calculator_iv(
            String::from("e + 8 - a + 5"),
            vec![String::from("e")],
            vec![1]
        ),
        vec![String::from("-1*a"), String::from("14")]
    );
    assert_eq!(
        basic_calculator_iv(
            String::from("e - 8 + temperature - pressure"),
            vec![String::from("e"), String::from("temperature")],
            vec![1, 12]
        ),
        vec![String::from("-1*pressure"), String::from("5")]
    );
    assert_eq!(
        basic_calculator_iv(
            String::from("(e + 8) * (e - 8)"),
            vec![String::new()],
            vec![]
        ),
        vec![String::from("1*e*e"), String::from("-64")]
    );
    assert_eq!(
        basic_calculator_iv(String::from("7 - 7"), vec![String::new()], vec![]),
        vec![String::new()]
    );
    assert_eq!(
        basic_calculator_iv(
            String::from("a * b * c + b * a * c * 4"),
            vec![String::new()],
            vec![]
        ),
        vec![String::from("5*a*b*c")]
    );
    assert_eq!(
        basic_calculator_iv(
            String::from("((a - b) * (b - c) + (c - a)) * ((a - b) + (b - c) * (c - a))"),
            vec![String::new()],
            vec![]
        ),
        vec![
            String::from("-1*a*a*b*b"),
            String::from("2*a*a*b*c"),
            String::from("-1*a*a*c*c"),
            String::from("1*a*b*b*b"),
            String::from("-1*a*b*b*c"),
            String::from("-1*a*b*c*c"),
            String::from("1*a*c*c*c"),
            String::from("-1*b*b*b*c"),
            String::from("2*b*b*c*c"),
            String::from("-1*b*c*c*c"),
            String::from("2*a*a*b"),
            String::from("-2*a*a*c"),
            String::from("-2*a*b*b"),
            String::from("2*a*c*c"),
            String::from("1*b*b*b"),
            String::from("-1*b*b*c"),
            String::from("1*b*c*c"),
            String::from("-1*c*c*c"),
            String::from("-1*a*a"),
            String::from("1*a*b"),
            String::from("1*a*c"),
            String::from("-1*b*c")
        ]
    );
}
