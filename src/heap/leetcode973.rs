// https://leetcode.com/problems/k-closest-points-to-origin/
pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}
// divide_and_conquer heap sort
#[test]
#[ignore]
fn test2_973() {
    assert_eq!(
        k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
        vec![vec![-2, 2]]
    );
    assert_eq!(
        k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
        vec![vec![3, 3], vec![-2, 4]]
    );
}
