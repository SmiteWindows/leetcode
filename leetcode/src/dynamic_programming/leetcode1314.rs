// https://leetcode-cn.com/problems/matrix-block-sum/
#![allow(clippy::many_single_char_names)]
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let n = mat.len();
    let m = mat[0].len();
    let mut prefix = vec![vec![0; m]; n];
    let mut res = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            prefix[i][j] += mat[i][j];
            if i > 0 {
                prefix[i][j] += prefix[i - 1][j];
            }
            if j > 0 {
                prefix[i][j] += prefix[i][j - 1];
            }
            if i > 0 && j > 0 {
                prefix[i][j] -= prefix[i - 1][j - 1];
            }
        }
    }
    for (i, resi) in res.iter_mut().enumerate().take(n) {
        for (j, resij) in resi.iter_mut().enumerate().take(m) {
            let l = j as i32 - k;
            let r = j as i32 + k;
            let t = i as i32 - k;
            let b = i as i32 + k;
            let l = if l < 0 { 0 } else { l as usize };
            let r = if r >= m as i32 { m - 1 } else { r as usize };
            let t = if t < 0 { 0 } else { t as usize };
            let b = if b >= n as i32 { n - 1 } else { b as usize };
            *resij = sum(t, l, b, r, &prefix);
        }
    }
    res
}

fn sum(t: usize, l: usize, b: usize, r: usize, prefix: &[Vec<i32>]) -> i32 {
    let mut res = prefix[b][r];
    if l > 0 {
        res -= prefix[b][l - 1];
    }
    if t > 0 {
        res -= prefix[t - 1][r];
    }
    if l > 0 && t > 0 {
        res += prefix[t - 1][l - 1];
    }
    res
}
// dynamic_programming
#[test]
fn test1_1314() {
    use leetcode_prelude::vec2;
    assert_eq!(
        matrix_block_sum(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 1),
        vec2![[12, 21, 16], [27, 45, 33], [24, 39, 28]]
    );
    assert_eq!(
        matrix_block_sum(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 2),
        vec2![[45, 45, 45], [45, 45, 45], [45, 45, 45]]
    );
}
