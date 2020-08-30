// https://leetcode-cn.com/problems/numbers-at-most-n-given-digit-set/
pub fn at_most_n_given_digit_set(d: Vec<String>, n: i32) -> i32 {
    todo!()
}
// math dynamic_programming
#[test]
#[ignore]
fn test2_902() {
    assert_eq!(
        at_most_n_given_digit_set(
            vec![
                String::from("1"),
                String::from("3"),
                String::from("5"),
                String::from("7")
            ],
            100
        ),
        20
    );
    assert_eq!(
        at_most_n_given_digit_set(
            vec![String::from("1"), String::from("4"), String::from("9")],
            1000000000
        ),
        29523
    );
}
