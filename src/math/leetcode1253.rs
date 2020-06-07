// https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/
// Runtime: 36 ms
// Memory Usage: 3.8 MB
pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut upper = upper;
    let mut lower = lower;
    let n = colsum.len();
    let mut res = vec![vec![0; n]; 2];
    for j in 0..n {
        match colsum[j] {
            2 => {
                res[0][j] = 1;
                res[1][j] = 1;
                upper -= 1;
                lower -= 1;
            }
            1 => {
                if upper >= lower {
                    res[0][j] = 1;
                    upper -= 1;
                } else {
                    res[1][j] = 1;
                    lower -= 1;
                }
            }
            _ => {}
        }
    }
    if upper == 0 && lower == 0 {
        res
    } else {
        vec![]
    }
}
// math greedy
#[test]
fn test1_1253() {
    assert_eq!(
        reconstruct_matrix(2, 1, vec![1, 1, 1]),
        vec![vec![1, 1, 0], vec![0, 0, 1]]
    );
    assert_eq!(reconstruct_matrix(2, 3, vec![2, 2, 1, 1]), vec![vec![]]);
    assert_eq!(
        reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
        vec![
            vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
        ]
    );
}
