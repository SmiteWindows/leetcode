// https://leetcode-cn.com/problems/set-matrix-zeroes/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
use std::collections::HashSet;
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut row = HashSet::new();
    let mut col = HashSet::new();
    let n = matrix.len();
    let m = matrix[0].len();
    for (i, mi) in matrix.iter().enumerate().take(n) {
        for (j, &mij) in mi.iter().enumerate().take(m) {
            if mij == 0 {
                row.insert(i);
                col.insert(j);
            }
        }
    }
    for (i, mi) in matrix.iter_mut().enumerate().take(n) {
        for (j, mij) in mi.iter_mut().enumerate().take(m) {
            if row.contains(&i) || col.contains(&j) {
                *mij = 0;
            }
        }
    }
}
// array
#[test]
fn test1_73() {
    use leetcode_prelude::vec2;
    let mut nums1 = vec2![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    set_zeroes(&mut nums1);
    assert_eq!(nums1, vec2![[1, 0, 1], [0, 0, 0], [1, 0, 1]]);

    let mut nums2 = vec2![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
    set_zeroes(&mut nums2);
    assert_eq!(nums2, vec2![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
}
