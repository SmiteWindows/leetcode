// https://leetcode-cn.com/problems/remove-covered-intervals/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Reverse;
pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_by_key(|v| (v[0], Reverse(v[1])));
    let mut r = -1;
    let mut res = 0;
    for interval in intervals {
        let interval = &interval;
        if interval[1] <= r {
            continue;
        } else {
            r = interval[1];
            res += 1;
        }
    }
    res
}
// line_sweep
#[test]
fn test1_1288() {
    use leetcode_prelude::vec2;
    assert_eq!(remove_covered_intervals(vec2![[1, 4], [3, 6], [2, 8]]), 2);
}
