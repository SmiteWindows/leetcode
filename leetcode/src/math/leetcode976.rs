// https://leetcode-cn.com/problems/largest-perimeter-triangle/
// Runtime: 8 ms
// Memory Usage: 2 MB
pub fn largest_perimeter(a: Vec<i32>) -> i32 {
    let mut a = a;
    let n = a.len();
    a.sort_unstable();
    for i in (0..=n - 3).rev() {
        if a[i] + a[i + 1] > a[i + 2] {
            return a[i] + a[i + 1] + a[i + 2];
        }
    }
    0
}
// math sort
#[test]
fn test1_976() {
    assert_eq!(largest_perimeter(vec![2, 1, 2]), 5);
    assert_eq!(largest_perimeter(vec![1, 2, 1]), 0);
    assert_eq!(largest_perimeter(vec![3, 2, 3, 4]), 10);
    assert_eq!(largest_perimeter(vec![3, 6, 2, 3]), 8);
}
