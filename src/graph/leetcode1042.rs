// https://leetcode.com/problems/flower-planting-with-no-adjacent/
pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1042() {
    assert_eq!(
        garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
        vec![1, 2, 3]
    );
    assert_eq!(
        garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 1, 2]
    );
    assert_eq!(
        garden_no_adj(
            4,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 3],
                vec![2, 4]
            ]
        ),
        vec![1, 2, 3, 4]
    );
}
