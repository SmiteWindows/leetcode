// https://leetcode-cn.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/
// Runtime: 16 ms
// Memory Usage: 2.8 MB
pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let mut prefix = vec![vec![0; m + 1]; n + 1];
    let mut k = 1;
    for i in 0..n {
        for j in 0..m {
            prefix[i][j] = mat[i][j];
            if i > 0 {
                prefix[i][j] += prefix[i - 1][j];
            }
            if j > 0 {
                prefix[i][j] += prefix[i][j - 1];
            }
            if i > 0 && j > 0 {
                prefix[i][j] -= prefix[i - 1][j - 1];
            }
            if i >= k - 1 && j >= k - 1 {
                let mut sum = prefix[i][j];
                if i >= k {
                    sum -= prefix[i - k][j];
                }
                if j >= k {
                    sum -= prefix[i][j - k];
                }
                if i >= k && j >= k {
                    sum += prefix[i - k][j - k];
                }
                if sum <= threshold {
                    k += 1;
                }
            }
        }
    }
    (k - 1) as i32
}
// binary_search array
#[test]
fn test1_1292() {
    use leetcode_prelude::vec2;
    assert_eq!(
        max_side_length(
            vec2![
                [1, 1, 3, 2, 4, 3, 2],
                [1, 1, 3, 2, 4, 3, 2],
                [1, 1, 3, 2, 4, 3, 2]
            ],
            4
        ),
        2
    );
    assert_eq!(
        max_side_length(
            vec2![
                [2, 2, 2, 2, 2],
                [2, 2, 2, 2, 2],
                [2, 2, 2, 2, 2],
                [2, 2, 2, 2, 2],
                [2, 2, 2, 2, 2]
            ],
            1
        ),
        0
    );
    assert_eq!(
        max_side_length(
            vec2![[1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
            6
        ),
        3
    );
    assert_eq!(
        max_side_length(
            vec2![
                [18, 70],
                [61, 1],
                [25, 85],
                [14, 40],
                [11, 96],
                [97, 96],
                [63, 45]
            ],
            40184
        ),
        2
    );
}
