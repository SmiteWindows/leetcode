// https://leetcode-cn.com/problems/get-equal-substrings-within-budget/
// Runtime: 0 ms
// Memory Usage: 3.5 MB
pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut cost = vec![0; n];
    for i in 0..n {
        cost[i] = (s[i] as i32 - t[i] as i32).abs();
    }
    let mut start = 0;
    let mut end = 0;
    let mut res = 0;
    let mut sum = 0;
    while end < n {
        if sum <= max_cost {
            sum += cost[end];
            end += 1;
        } else {
            sum -= cost[start];
            start += 1;
        }
        if sum <= max_cost {
            res = res.max(end - start);
        }
    }
    res as i32
}
// sliding_window array
#[test]
fn test2_1208() {
    assert_eq!(
        equal_substring("abcd"), "bcdf"), 3),
        3
    );
    assert_eq!(
        equal_substring("abcd"), "cdef"), 3),
        1
    );
    assert_eq!(
        equal_substring("abcd"), "acde"), 0),
        1
    );
}
