// https://leetcode.com/problems/max-points-on-a-line/
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// math hash_table
#[test]
#[ignore]
fn test2_149() {
    assert_eq!(max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
    assert_eq!(
        max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4]
        ]),
        4
    );
}
