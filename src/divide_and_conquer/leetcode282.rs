// https://leetcode.com/problems/expression-add-operators/
pub fn add_operators(num: String, target: i32) -> Vec<String> {
    todo!()
}
// divide_and_conquer
#[test]
#[ignore]
fn test1_241() {
    assert_eq!(
        add_operators(String::from("123"), 6),
        vec![String::from("1+2+3"), String::from("1*2*3")]
    );
    assert_eq!(
        add_operators(String::from("232"), 8),
        vec![String::from("2*3+2"), String::from("2+3*2")]
    );
    assert_eq!(
        add_operators(String::from("105"), 5),
        vec![String::from("1*0+5"), String::from("10-5")]
    );
    assert_eq!(
        add_operators(String::from("00"), 0),
        vec![
            String::from("0+0"),
            String::from("0-0"),
            String::from("0*0")
        ]
    );
    assert_eq!(
        add_operators(String::from("3456237490"), 9191),
        vec![String::new()]
    );
}
