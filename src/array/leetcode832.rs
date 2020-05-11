// https://leetcode.com/problems/flipping-an-image/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut a = a;
    let n = a.len();
    for i in 0..n {
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            a[i].swap(l, r);
            l += 1;
            r -= 1;
        }
        for j in 0..n {
            a[i][j] = 1 - a[i][j];
        }
    }
    a
}
// array
#[test]
fn test1_832() {
    assert_eq!(
        flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    );
    assert_eq!(
        flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0]
        ]
    );
}
