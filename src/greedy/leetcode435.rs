// https://leetcode.com/problems/non-overlapping-intervals/
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_435() {
    assert_eq!(
        erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
    assert_eq!(
        erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );
    assert_eq!(erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]), 0);
}
