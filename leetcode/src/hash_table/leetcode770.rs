// https://leetcode-cn.com/problems/basic-calculator-iv/
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
fn test3_770() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        basic_calculator_iv("e + 8 - a + 5".to_string(), vec_string!["e"], vec![1]),
        vec_string!["-1*a", "14"]
    );
    assert_eq!(
        basic_calculator_iv(
            "e - 8 + temperature - pressure".to_string(),
            vec_string!["e", "temperature"],
            vec![1, 12]
        ),
        vec_string!["-1*pressure", "5"]
    );
    assert_eq!(
        basic_calculator_iv("(e + 8) * (e - 8)".to_string(), vec_string![], vec![]),
        vec_string!["1*e*e", "-64"]
    );
    assert_eq!(
        basic_calculator_iv("7 - 7".to_string(), vec_string![], vec![]),
        vec_string![]
    );
    assert_eq!(
        basic_calculator_iv(
            "a * b * c + b * a * c * 4".to_string(),
            vec_string![],
            vec![]
        ),
        vec_string!["5*a*b*c"]
    );
    assert_eq!(
        basic_calculator_iv(
            "((a - b) * (b - c) + (c - a)) * ((a - b) + (b - c) * (c - a))".to_string(),
            vec_string![],
            vec![]
        ),
        vec_string![
            "-1*a*a*b*b",
            "2*a*a*b*c",
            "-1*a*a*c*c",
            "1*a*b*b*b",
            "-1*a*b*b*c",
            "-1*a*b*c*c",
            "1*a*c*c*c",
            "-1*b*b*b*c",
            "2*b*b*c*c",
            "-1*b*c*c*c",
            "2*a*a*b",
            "-2*a*a*c",
            "-2*a*b*b",
            "2*a*c*c",
            "1*b*b*b",
            "-1*b*b*c",
            "1*b*c*c",
            "-1*c*c*c",
            "-1*a*a",
            "1*a*b",
            "1*a*c",
            "-1*b*c"
        ]
    );
}
