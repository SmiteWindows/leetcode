// https://leetcode-cn.com/problems/monotonic-array/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
pub fn is_monotonic(a: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let n = a.len();
    for i in 1..n {
        if a[i] > a[i - 1] {
            decreasing = false;
        }
        if a[i] < a[i - 1] {
            increasing = false;
        }
    }
    increasing || decreasing
}
// array
#[test]
fn test1_896() {
    assert_eq!(is_monotonic(vec![1, 2, 2, 3]), true);
    assert_eq!(is_monotonic(vec![6, 5, 4, 4]), true);
    assert_eq!(is_monotonic(vec![1, 3, 2]), false);
    assert_eq!(is_monotonic(vec![1, 2, 4, 5]), true);
    assert_eq!(is_monotonic(vec![1, 1, 1]), true);
}
