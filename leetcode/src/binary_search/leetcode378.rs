// https://leetcode-cn.com/problems/kth-smallest-element-in-a-sorted-matrix/
#![allow(clippy::many_single_char_names)]
// Runtime: 4 ms
// Memory Usage: 2.7 MB
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = matrix.len();
    let mut l = matrix[0][0];
    let mut h = matrix[n - 1][n - 1];
    while l < h {
        let m = l + (h - l) / 2;
        let mut count = 0;
        let mut i = n as i32 - 1;
        for mj in matrix.iter().take(n) {
            while i >= 0 && mj[i as usize] > m {
                i -= 1;
            }
            count += i + 1;
        }
        if count < k {
            l = m + 1;
        } else {
            h = m;
        }
    }
    l
}
// binary_search heap
#[test]
fn test1_378() {
    assert_eq!(
        kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
        13
    );
}
