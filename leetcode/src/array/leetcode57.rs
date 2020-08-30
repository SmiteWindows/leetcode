// https://leetcode-cn.com/problems/insert-interval/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.push(new_interval);
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
fn test2_57() {
    assert_eq!(
        insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![vec![1, 5], vec![6, 9]]
    );
    assert_eq!(
        insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        ),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
}
