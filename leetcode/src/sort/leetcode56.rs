// https://leetcode.com/problems/merge-intervals/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|a| a[0]);
    let mut res = vec![];
    let mut temp: Option<Vec<i32>> = None;
    for v in intervals {
        if let Some(t) = temp {
            if v[0] <= t[1] {
                temp = Some(vec![t[0], t[1].max(v[1])]);
            } else {
                temp = Some(v);
                res.push(t);
            }
        } else {
            temp = Some(v);
        }
    }
    if let Some(t) = temp {
        res.push(t);
    }
    res
}
// array sort
#[test]
fn test1_56() {
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
    assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
}
