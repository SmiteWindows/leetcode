// https://leetcode.com/problems/valid-mountain-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn valid_mountain_array(a: Vec<i32>) -> bool {
    let n = a.len();
    if n < 3 {
        return false;
    }
    let mut i = 0;
    while i + 1 < n && a[i] < a[i + 1] {
        i += 1;
    }
    if i == 0 || i == n - 1 {
        return false;
    }
    while i + 1 < n && a[i] > a[i + 1] {
        i += 1;
    }
    i == n - 1
}
// array
#[test]
fn test1_941() {
    assert_eq!(valid_mountain_array(vec![2, 1]), false);
    assert_eq!(valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(valid_mountain_array(vec![0, 3, 2, 1]), true);
}
