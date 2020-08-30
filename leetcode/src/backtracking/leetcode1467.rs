// https://leetcode-cn.com/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls/
pub fn get_probability(balls: Vec<i32>) -> f64 {
    todo!()
}
// math backtracking
#[test]
#[ignore]
fn test2_1467() {
    assert_eq!(get_probability(vec![1, 1]), 1.00000);
    assert_eq!(get_probability(vec![2, 1, 1]), 0.66667);
    assert_eq!(get_probability(vec![1, 2, 1, 2]), 0.60000);
    assert_eq!(get_probability(vec![3, 2, 1]), 0.30000);
    assert_eq!(get_probability(vec![6, 6, 6, 6, 6, 6]), 0.90327);
}