// https://leetcode.com/problems/image-smoother/
// Runtime: 16 ms
// Memory Usage: 2.3 MB
pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let height = m.len();
    let weight = m[0].len();
    let mut res = vec![vec![0; weight]; height];
    for (i, row) in res.iter_mut().enumerate().take(height) {
        for (j, col) in row.iter_mut().enumerate().take(weight) {
            *col = smooth(&m, i, j, height, weight);
        }
    }
    res
}

fn smooth(m: &[Vec<i32>], r: usize, c: usize, height: usize, weight: usize) -> i32 {
    let mut sum = 0;
    let mut n = 0;
    if r > 0 && c > 0 {
        sum += m[r - 1][c - 1];
        n += 1;
    }
    if r > 0 {
        sum += m[r - 1][c];
        n += 1;
    }
    if r > 0 && c < weight - 1 {
        sum += m[r - 1][c + 1];
        n += 1;
    }
    if c > 0 {
        sum += m[r][c - 1];
        n += 1;
    }
    sum += m[r][c];
    n += 1;
    if c < weight - 1 {
        sum += m[r][c + 1];
        n += 1;
    }
    if r < height - 1 && c > 0 {
        sum += m[r + 1][c - 1];
        n += 1;
    }
    if r < height - 1 {
        sum += m[r + 1][c];
        n += 1;
    }
    if r < height - 1 && c < weight - 1 {
        sum += m[r + 1][c + 1];
        n += 1;
    }
    sum / n
}
// array
#[test]
fn test1_661() {
    assert_eq!(
        image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
}
