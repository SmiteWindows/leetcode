// https://leetcode.com/problems/longest-mountain-in-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn longest_mountain(a: Vec<i32>) -> i32 {
    let n = a.len();
    if n == 0 {
        return 0;
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for i in 1..n {
        if a[i] > a[i - 1] {
            left[i] = left[i - 1] + 1;
        }
    }
    for i in (0..n - 1).rev() {
        if a[i] > a[i + 1] {
            right[i] = right[i + 1] + 1;
        }
    }
    let mut res = 0;
    for i in 0..n {
        if left[i] > 0 && right[i] > 0 {
            res = res.max(left[i] + right[i] + 1);
        }
    }
    res
}
// two_pointers
#[test]
fn test1_845() {
    assert_eq!(longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    assert_eq!(longest_mountain(vec![2, 2, 2]), 0);
}
