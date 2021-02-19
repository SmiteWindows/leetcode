// https://leetcode-cn.com/problems/longest-turbulent-subarray/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let n = a.len();
    let mut res = 1;
    let mut inc = 1;
    let mut dec = 1;
    for i in 1..n {
        match (a[i] - a[i - 1]).cmp(&0) {
            Equal => {
                inc = 1;
                dec = 1;
            }
            Less => {
                inc = dec + 1;
                dec = 1;
            }
            Greater => {
                dec = inc + 1;
                inc = 1;
            }
        }
        res = res.max(inc.max(dec));
    }
    res
}
// sliding_window array dynamic_programming
#[test]
fn test3_978() {
    assert_eq!(max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    assert_eq!(max_turbulence_size(vec![4, 8, 12, 16]), 2);
    assert_eq!(max_turbulence_size(vec![100]), 1);
}
