// https://leetcode-cn.com/problems/non-overlapping-intervals/
// Runtime: 0 ms
// Memory Usage: 2.5 MB
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    intervals.sort_by_key(|v| v[1]);
    let mut end = intervals[0][1];
    let mut res = 0;
    for interval in intervals.iter().skip(1) {
        if interval[0] < end {
            res += 1;
        } else {
            end = interval[1];
        }
    }
    res
}
// greedy
#[test]
fn test1_435() {
    use leetcode_prelude::vec2;
    assert_eq!(
        erase_overlap_intervals(vec2![[1, 2], [2, 3], [3, 4], [1, 3]]),
        1
    );
    assert_eq!(erase_overlap_intervals(vec2![[1, 2], [1, 2], [1, 2]]), 2);
    assert_eq!(erase_overlap_intervals(vec2![[1, 2], [2, 3]]), 0);
}
