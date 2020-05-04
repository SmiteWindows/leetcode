// https://leetcode.com/problems/erect-the-fence/
pub fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// geometry
#[test]
#[ignore]
fn test1_587() {
    assert_eq!(
        outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2]
        ]),
        vec![vec![1, 1], vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4]]
    );
    assert_eq!(
        outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]),
        vec![vec![1, 2], vec![2, 2], vec![4, 2]]
    );
}
