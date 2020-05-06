// https://leetcode.com/problems/bag-of-tokens/
pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_955() {
    assert_eq!(bag_of_tokens_score(vec![100], 50), 0);
    assert_eq!(bag_of_tokens_score(vec![100, 200], 150), 1);
    assert_eq!(bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
}
