// https://leetcode.com/problems/matrix-block-sum/
pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1314() {
    assert_eq!(
        matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]
    );
    assert_eq!(
        matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 2),
        vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]]
    );
}
