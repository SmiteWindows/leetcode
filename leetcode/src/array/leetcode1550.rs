// https://leetcode.com/problems/three-consecutive-odds/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    arr.windows(3)
        .any(|w| w[0] % 2 == 1 && w[1] % 2 == 1 && w[2] % 2 == 1)
}
#[test]
fn test1_1550() {
    assert_eq!(three_consecutive_odds(vec![2, 6, 4, 1]), false);
    assert_eq!(
        three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
        true
    );
}
