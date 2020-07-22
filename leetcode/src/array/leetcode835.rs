// https://leetcode.com/problems/image-overlap/
#![allow(clippy::many_single_char_names)]
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
    let n = a.len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            res = res.max(translate(i, j, &a, &b, n));
            res = res.max(translate(i, j, &b, &a, n));
        }
    }
    res
}

fn translate(x: usize, y: usize, a: &[Vec<i32>], b: &[Vec<i32>], n: usize) -> i32 {
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if i + x < n && j + y < n {
                res += a[i + x][j + y] * b[i][j];
            }
        }
    }
    res
}
// array
#[test]
fn test1_835() {
    assert_eq!(
        largest_overlap(
            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
        ),
        3
    );
}
