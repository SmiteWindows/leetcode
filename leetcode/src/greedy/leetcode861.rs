// https://leetcode.com/problems/score-after-flipping-matrix/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
    let n = a.len();
    let m = a[0].len();
    let mut res = n << (m - 1);
    for j in 1..m {
        let mut x = sum_col(j, &a, n);
        x = x.max(n - x);
        res += x << (m - 1 - j);
    }
    res as i32
}

fn sum_col(j: usize, a: &[Vec<i32>], n: usize) -> usize {
    let mut res = 0;
    for ai in a.iter().take(n) {
        res += if ai[j] == ai[0] { 1 } else { 0 };
    }
    res as usize
}
// greedy
#[test]
fn test1_861() {
    assert_eq!(
        matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
        39
    );
}
