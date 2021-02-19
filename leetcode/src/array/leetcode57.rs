// https://leetcode-cn.com/problems/insert-interval/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    intervals.push(new_interval);
    intervals.sort_unstable_by_key(|a| a[0]);
    let mut res = Vec::new();
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
    use leetcode_prelude::vec2;
    assert_eq!(
        insert(vec2![[1, 3], [6, 9]], vec![2, 5]),
        vec2![[1, 5], [6, 9]]
    );
    assert_eq!(
        insert(vec2![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], vec![4, 8]),
        vec2![[1, 2], [3, 10], [12, 16]]
    );
}
