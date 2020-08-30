// https://leetcode-cn.com/problems/best-sightseeing-pair/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut maxs = vec![];
    let mut prev_max = 0;
    let mut res = 0;
    for (i, ai) in a.iter().enumerate().take(n) {
        prev_max = prev_max.max(i as i32 + ai);
        maxs.push(prev_max);
    }
    for i in 1..n {
        res = res.max(maxs[i - 1] + a[i] - i as i32);
    }
    res
}
// array
#[test]
fn test1_1014() {
    assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
}
