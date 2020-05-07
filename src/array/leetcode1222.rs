// https://leetcode.com/problems/queens-that-can-attack-the-king/
pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1222() {
    assert_eq!(
        queens_attackthe_king(
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![4, 0],
                vec![0, 4],
                vec![3, 3],
                vec![2, 4]
            ],
            vec![0, 0]
        ),
        vec![vec![0, 1], vec![1, 0], vec![3, 3]]
    );
    assert_eq!(
        queens_attackthe_king(
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![3, 5],
                vec![4, 4],
                vec![4, 5]
            ],
            vec![3, 3]
        ),
        vec![vec![2, 2], vec![3, 4], vec![4, 4]]
    );
    assert_eq!(
        queens_attackthe_king(
            vec![
                vec![5, 6],
                vec![7, 7],
                vec![2, 1],
                vec![0, 7],
                vec![1, 6],
                vec![5, 1],
                vec![3, 7],
                vec![0, 3],
                vec![4, 0],
                vec![1, 2],
                vec![6, 3],
                vec![5, 0],
                vec![0, 4],
                vec![2, 2],
                vec![1, 1],
                vec![6, 4],
                vec![5, 4],
                vec![0, 0],
                vec![2, 6],
                vec![4, 5],
                vec![5, 2],
                vec![1, 4],
                vec![7, 5],
                vec![2, 3],
                vec![0, 5],
                vec![4, 2],
                vec![1, 0],
                vec![2, 7],
                vec![0, 1],
                vec![4, 6],
                vec![6, 1],
                vec![0, 6],
                vec![4, 3],
                vec![1, 7]
            ],
            vec![3, 4]
        ),
        vec![
            vec![2, 3],
            vec![1, 4],
            vec![1, 6],
            vec![3, 7],
            vec![4, 3],
            vec![5, 4],
            vec![4, 5]
        ]
    );
}
