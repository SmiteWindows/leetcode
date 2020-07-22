// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/
#![allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    let mut mat = mat;
    let n = mat.len();
    let m = mat[0].len();
    let mut res = usize::MAX;
    dfs(0, 0, &mut mat, &mut res, n, m);
    if res == usize::MAX {
        -1
    } else {
        res as i32
    }
}

fn dfs(start: usize, k: usize, mat: &mut Vec<Vec<i32>>, min: &mut usize, n: usize, m: usize) {
    if start == n * m {
        if ones(mat, n, m) == 0 {
            *min = (*min).min(k);
        }
    } else {
        let r = start / m;
        let c = start % m;
        flip(r, c, mat, n, m);
        dfs(start + 1, k + 1, mat, min, n, m);
        flip(r, c, mat, n, m);
        dfs(start + 1, k, mat, min, n, m);
    }
}

fn flip(i: usize, j: usize, mat: &mut Vec<Vec<i32>>, n: usize, m: usize) {
    mat[i][j] = 1 - mat[i][j];
    if i > 0 {
        mat[i - 1][j] = 1 - mat[i - 1][j];
    }
    if j > 0 {
        mat[i][j - 1] = 1 - mat[i][j - 1];
    }
    if i + 1 < n {
        mat[i + 1][j] = 1 - mat[i + 1][j];
    }
    if j + 1 < m {
        mat[i][j + 1] = 1 - mat[i][j + 1];
    }
}

fn ones(mat: &[Vec<i32>], n: usize, m: usize) -> usize {
    let mut res = 0;
    for mi in mat.iter().take(n) {
        for &m in mi.iter().take(m) {
            if m == 1 {
                res += 1;
            }
        }
    }
    res
}
// breadth_first_search
#[test]
fn test1_1284() {
    assert_eq!(min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
    assert_eq!(min_flips(vec![vec![0]]), 0);
    assert_eq!(
        min_flips(vec![vec![1, 1, 1], vec![1, 0, 1], vec![0, 0, 0]]),
        6
    );
    assert_eq!(min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
}
