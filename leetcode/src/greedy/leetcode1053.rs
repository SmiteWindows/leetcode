// https://leetcode-cn.com/problems/previous-permutation-with-one-swap/
// Runtime: 12 ms
// Memory Usage: 2.3 MB
pub fn prev_perm_opt1(a: Vec<i32>) -> Vec<i32> {
    let mut a = a;
    let n = a.len();
    if n < 2 {
        return a;
    }
    let mut i = n - 2;
    while i > 0 && a[i] <= a[i + 1] {
        i -= 1;
    }
    if i == 0 && a[0] <= a[1] {
        return a;
    }
    let mut j = n - 1;
    while a[j] >= a[i] || a[j] == a[j - 1] {
        j -= 1;
    }
    a.swap(i, j);
    a
}
// greedy array
#[test]
fn test1_1053() {
    assert_eq!(prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
    assert_eq!(prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
    assert_eq!(prev_perm_opt1(vec![1, 9, 4, 6, 7]), vec![1, 7, 4, 6, 9]);
    assert_eq!(prev_perm_opt1(vec![3, 1, 1, 3]), vec![1, 3, 1, 3]);
}
