// https://leetcode-cn.com/problems/lucky-numbers-in-a-matrix/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut mins: Vec<i32> = vec![i32::MAX; m];
    let mut maxs: Vec<i32> = vec![0; n];
    let mut res = Vec::new();
    for (i, min) in mins.iter_mut().enumerate().take(m) {
        for (j, max) in maxs.iter_mut().enumerate().take(n) {
            *min = matrix[i][j].min(*min);
            *max = matrix[i][j].max(*max);
        }
    }
    for (i, min) in mins.iter().enumerate().take(m) {
        for (j, max) in maxs.iter().enumerate().take(n) {
            if *min == matrix[i][j] && *max == matrix[i][j] {
                res.push(matrix[i][j]);
            }
        }
    }
    res
}
// array
#[test]
fn test1_1380() {
    use leetcode_prelude::vec2;
    assert_eq!(
        lucky_numbers(vec2![[3, 7, 8], [9, 11, 13], [15, 16, 17]]),
        vec![15]
    );
    assert_eq!(
        lucky_numbers(vec2![[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]]),
        vec![12]
    );
    assert_eq!(lucky_numbers(vec2![[7, 8], [1, 2]]), vec![7]);
}
