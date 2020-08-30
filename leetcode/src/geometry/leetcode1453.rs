// https://leetcode-cn.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard/
pub fn num_points(points: Vec<Vec<i32>>, r: i32) -> i32 {
    todo!()
}
// geometry
#[test]
#[ignore]
fn test1_1453() {
    assert_eq!(
        num_points(vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]], 2),
        4
    );
    assert_eq!(
        num_points(
            vec![
                vec![-3, 0],
                vec![3, 0],
                vec![2, 6],
                vec![5, 4],
                vec![0, 9],
                vec![7, 8]
            ],
            5
        ),
        5
    );
    assert_eq!(
        num_points(vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]], 1),
        1
    );
    assert_eq!(
        num_points(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![1, -1],
                vec![2, 3],
                vec![4, 1],
                vec![1, 3]
            ],
            2
        ),
        4
    );
}
