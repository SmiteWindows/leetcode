// https://leetcode-cn.com/problems/maximum-width-ramp/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn max_width_ramp(a: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let n = a.len();
    for i in 0..n {
        if let Some(&j) = stack.last() {
            if a[i] < a[j] {
                stack.push(i);
            }
        } else {
            stack.push(i);
        }
    }
    let mut res = 0;
    for i in (0..n).rev() {
        while let Some(&j) = stack.last() {
            if a[j] <= a[i] {
                res = res.max(i - j);
                stack.pop();
            } else {
                break;
            }
        }
    }
    res as i32
}
// array
#[test]
fn test1_962() {
    assert_eq!(max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    assert_eq!(max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
}
