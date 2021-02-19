// https://leetcode-cn.com/problems/merge-intervals/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
fn test1_56() {
    use leetcode_prelude::vec2;
    assert_eq!(
        merge(vec2![[1, 3], [2, 6], [8, 10], [15, 18]]),
        vec2![[1, 6], [8, 10], [15, 18]]
    );
    assert_eq!(merge(vec2![[1, 4], [4, 5]]), vec2![[1, 5]]);
}
