// https://leetcode-cn.com/problems/find-n-unique-integers-sum-up-to-zero/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn sum_zero(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut res = vec![0; n];
    let mut sum: i32 = 0;
    for (i, val) in res.iter_mut().enumerate().take(n - 1) {
        sum -= i as i32;
        *val = i as i32;
    }
    res[n - 1] = sum;
    res
}
// array
#[test]
fn test1_1304() {
    assert_eq!(sum_zero(5), vec![0, 1, 2, 3, -6]);
    // assert_eq!(sum_zero(5), vec![-7, -1, 1, 3, 4]);
    // assert_eq!(sum_zero(5), vec![-5, -1, 1, 2, 3]);
    // assert_eq!(sum_zero(5), vec![-3, -1, 2, -2, 4]);
    assert_eq!(sum_zero(3), vec![0, 1, -1]);
    // assert_eq!(sum_zero(3), vec![-1, 0, 1]);
    assert_eq!(sum_zero(1), vec![0]);
}
