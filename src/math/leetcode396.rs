// https://leetcode.com/problems/rotate-function
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn max_rotate_function(a: Vec<i32>) -> i32 {
    let n = a.len();
    if n == 0 {
        return 0;
    }
    let sum = a.iter().sum::<i32>();
    let mut f = 0;
    for (i, ai) in a.iter().enumerate() {
        f += i as i32 * ai;
    }
    let mut res = f;
    for (i, ai) in a.iter().enumerate().rev() {
        f = f + sum - n as i32 * ai;
        res = res.max(f);
    }
    res
}
// math
#[test]
fn test1_396() {
    assert_eq!(max_rotate_function(vec![4, 3, 2, 6]), 26);
}
