// https://leetcode.com/problems/edit-distance/
pub fn min_distance(word1: String, word2: String) -> i32 {
    todo!()
}
// dynamic_programming string
#[test]
#[ignore]
fn test2_72() {
    assert_eq!(min_distance(String::from("horse"), String::from("ros")), 3);
    assert_eq!(
        min_distance(String::from("intention"), String::from("execution")),
        5
    );
}
