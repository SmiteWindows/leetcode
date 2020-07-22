// https://leetcode.com/problems/count-submatrices-with-all-ones/
// Runtime: 16 ms
// Memory Usage: 2.3 MB
pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            let mut limit = m;
            for mr in mat.iter().take(n).skip(i) {
                for (c, &mrc) in mr.iter().enumerate().take(m.min(limit)).skip(j) {
                    if mrc == 1 {
                        res += 1;
                    } else {
                        limit = c;
                        break;
                    }
                }
            }
        }
    }
    res
}
// dynamic_programming
#[test]
fn test1_1504() {
    assert_eq!(
        num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        13
    );
    assert_eq!(
        num_submat(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]]),
        24
    );
    assert_eq!(num_submat(vec![vec![1, 1, 1, 1, 1, 1]]), 21);
    assert_eq!(
        num_submat(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]),
        5
    );
}
