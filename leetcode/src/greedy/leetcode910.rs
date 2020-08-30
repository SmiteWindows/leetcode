// https://leetcode-cn.com/problems/smallest-range-ii/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn smallest_range_ii(a: Vec<i32>, k: i32) -> i32 {
    let mut a = a;
    let n = a.len();
    a.sort_unstable();
    let mut max = a[n - 1];
    let mut min = a[0];
    let mut res = max - min;
    for i in 0..n - 1 {
        max = max.max(a[i] + 2 * k);
        min = a[i + 1].min(a[0] + 2 * k);
        res = res.min(max - min);
    }
    res
}
// math greedy
#[test]
fn test2_910() {
    assert_eq!(smallest_range_ii(vec![1], 0), 0);
    assert_eq!(smallest_range_ii(vec![0, 10], 2), 6);
    assert_eq!(smallest_range_ii(vec![1, 3, 6], 3), 3);
}
