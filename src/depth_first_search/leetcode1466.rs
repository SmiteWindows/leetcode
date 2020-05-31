// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// tree depth_first_search
#[test]
#[ignore]
fn test2_1466() {
    assert_eq!(
        min_reorder(
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
        ),
        3
    );
    assert_eq!(
        min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
        2
    );
    assert_eq!(min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0);
}
