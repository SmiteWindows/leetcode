// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/
pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    todo!()
}
// greedy dynamic_programming
#[test]
#[ignore]
fn test1_1326() {
    assert_eq!(min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    assert_eq!(min_taps(3, vec![0, 0, 0, 0]), -1);
    assert_eq!(min_taps(7, vec![1, 2, 1, 0, 2, 1, 0, 1]), 3);
    assert_eq!(min_taps(8, vec![4, 0, 0, 0, 0, 0, 0, 0, 4]), 2);
    assert_eq!(min_taps(8, vec![4, 0, 0, 0, 4, 0, 0, 0, 4]), 1);
}
