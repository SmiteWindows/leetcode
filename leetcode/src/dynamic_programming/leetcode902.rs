// https://leetcode-cn.com/problems/numbers-at-most-n-given-digit-set/
pub fn at_most_n_given_digit_set(d: Vec<String>, n: i32) -> i32 {
    todo!()
}
// math dynamic_programming
#[test]
#[ignore]
fn test2_902() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        at_most_n_given_digit_set(vec_string!["1", "3", "5", "7"], 100),
        20
    );
    assert_eq!(
        at_most_n_given_digit_set(vec_string!["1", "4", "9"], 1000000000),
        29523
    );
}
