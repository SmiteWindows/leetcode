// https://leetcode.com/problems/global-and-local-inversions/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
    let n = a.len();
    for i in 0..n {
        if (a[i] - i as i32).abs() > 1 {
            return false;
        }
    }
    true
}
// math array
#[test]
fn test2_775() {
    assert_eq!(is_ideal_permutation(vec![1, 0, 2]), true);
    assert_eq!(is_ideal_permutation(vec![1, 2, 0]), false);
}
