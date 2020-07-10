// https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
    let mut a = a;
    let n = a.len();
    let l = l as usize;
    let m = m as usize;
    for i in 1..n {
        a[i] += a[i - 1];
    }
    let mut res = a[l + m - 1];
    let mut max_l = a[l - 1];
    let mut max_m = a[m - 1];
    for i in l + m..n {
        max_l = max_l.max(a[i - m] - a[i - m - l]);
        max_m = max_m.max(a[i - l] - a[i - l - m]);
        let last_l = a[i] - a[i - l];
        let last_m = a[i] - a[i - m];
        res = res.max((max_m + last_l).max(max_l + last_m));
    }
    res
}
// array
#[test]
fn test1_1031() {
    assert_eq!(
        max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
        20
    );
    assert_eq!(
        max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
        29
    );
    assert_eq!(
        max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
        31
    );
}
