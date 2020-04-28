// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// array dynamic_programming sliding_window
#[test]
#[ignore]
fn test1_1423() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    assert_eq!(max_score(vec![1, 1000, 1], 1), 1);
    assert_eq!(max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3), 202);
}
