// https://leetcode.com/problems/insert-interval/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut new_interval = new_interval;
    let mut res = vec![];
    for interval in intervals {
        if interval[0] < new_interval[0] {
            if interval[1] < new_interval[0] {
                res.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        } else if interval[0] > new_interval[1] {
            res.push(interval);
        } else {
            new_interval[0] = new_interval[0].min(interval[0]);
            new_interval[1] = new_interval[1].max(interval[1]);
        }
    }
    if let Err(i) = res.binary_search_by_key(&new_interval[0], |v| v[0]) {
        res.insert(i, new_interval);
    }
    res
}
// array sort
#[test]
fn test1_57() {
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
