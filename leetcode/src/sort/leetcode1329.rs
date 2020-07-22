// https://leetcode.com/problems/sort-the-matrix-diagonally/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut mat = mat;
    let n = mat.len();
    let m = mat[0].len();
    let mut hs: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();
    for (i, mi) in mat.iter().enumerate().take(n) {
        for (j, &mj) in mi.iter().enumerate().take(m) {
            hs.entry(i as i32 - j as i32).or_default().push(Reverse(mj));
        }
    }
    for (i, mi) in mat.iter_mut().enumerate().take(n) {
        for (j, mj) in mi.iter_mut().enumerate().take(m) {
            *mj = hs.entry(i as i32 - j as i32).or_default().pop().unwrap().0;
        }
    }
    mat
}
// sort array
#[test]
fn test1_1329() {
    assert_eq!(
        diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
        vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
    );
}
