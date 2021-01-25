// https://leetcode-cn.com/problems/reconstruct-a-2-row-binary-matrix/
// Runtime: 36 ms
// Memory Usage: 3.8 MB
pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = colsum.len();
    let mut res = vec![vec![0; n]; 2];
    for (j, &cs) in colsum.iter().enumerate().take(n) {
        match cs {
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
    use leetcode_prelude::vec2;
    assert_eq!(
        reconstruct_matrix(2, 1, vec![1, 1, 1]),
        vec2![[1, 1, 0], [0, 0, 1]]
    );
    assert_eq!(reconstruct_matrix(2, 3, vec![2, 2, 1, 1]), vec2![]);
    // assert_eq!(
    //     reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
    //     vec2![
    //         [1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
    //         [1, 0, 1, 0, 1, 0, 0, 1, 0, 1]
    //     ]
    // );
}
