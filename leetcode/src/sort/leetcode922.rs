// https://leetcode-cn.com/problems/sort-array-by-parity-ii/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
    let mut a = a;
    let n = a.len();
    let mut i: usize = 0;
    let mut j: usize = 1;
    while i < n && j < n {
        while i < n && a[i] % 2 == 0 {
            i += 2;
        }
        while j < n && a[j] % 2 == 1 {
            j += 2;
        }
        if i < n && j < n {
            a.swap(i, j);
        }
    }
    a
}
// sort array
#[test]
fn test1_922() {
    assert_eq!(sort_array_by_parity_ii(vec![4, 2, 5, 7]), vec![4, 5, 2, 7]);
}
