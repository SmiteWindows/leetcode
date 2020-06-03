// https://leetcode.com/problems/reshape-the-matrix/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    let m = nums[0].len();
    let r = r as usize;
    let c = c as usize;
    if n * m != r * c {
        return nums;
    }
    let mut res = vec![vec![0; c]; r];
    for i in 0..n {
        for j in 0..m {
            let k = i * m + j;
            res[k / c][k % c] = nums[i][j];
        }
    }
    res
}
// array
#[test]
fn test1_566() {
    assert_eq!(
        matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
    assert_eq!(
        matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );
}
