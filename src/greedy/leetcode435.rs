// https://leetcode.com/problems/non-overlapping-intervals/
// Runtime: 0 ms
// Memory Usage: 2.5 MB
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    if intervals.is_empty() {
        return 0;
    }
    let n = intervals.len();
    intervals.sort_by_key(|v| v[1]);
    let mut end = intervals[0][1];
    let mut res = 0;
    for i in 1..n {
        if intervals[i][0] < end {
            res += 1;
        } else {
            end = intervals[i][1];
        }
    }
    res
}
// greedy
#[test]
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
