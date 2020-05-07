// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/
pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1284() {
    assert_eq!(min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
    assert_eq!(min_flips(vec![vec![0]]), 0);
    assert_eq!(
        min_flips(vec![vec![1, 1, 1], vec![1, 0, 1], vec![0, 0, 0]]),
        6
    );
    assert_eq!(min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
}
