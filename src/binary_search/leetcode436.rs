// https://leetcode.com/problems/find-right-interval/
// Runtime: 16 ms
// Memory Usage: 2.9 MB
use std::collections::BTreeMap;
pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();
    let mut res = vec![-1; n];
    let mut btm = BTreeMap::new();
    for (i, interval) in intervals.iter().enumerate().take(n) {
        let l = interval[0];
        btm.insert(l, i);
    }
    for i in 0..n {
        let r = intervals[i][1];
        for (_, &j) in btm.range(r..).take(1) {
            res[i] = j as i32;
        }
    }
    res
}
// binary_search
#[test]
fn test1_436() {
    assert_eq!(find_right_interval(vec![vec![1, 2]]), vec![-1]);
    assert_eq!(
        find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        vec![-1, 0, 1]
    );
    assert_eq!(
        find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        vec![-1, 2, -1]
    );
}
