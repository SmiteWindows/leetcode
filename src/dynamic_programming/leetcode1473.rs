// https://leetcode.com/problems/paint-house-iii/
pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1473() {
    assert_eq!(
        min_cost(
            vec![0, 0, 0, 0, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1]
            ],
            5,
            2,
            3
        ),
        9
    );
    assert_eq!(
        min_cost(
            vec![0, 2, 1, 2, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1]
            ],
            5,
            2,
            3
        ),
        11
    );
    assert_eq!(
        min_cost(
            vec![0, 0, 0, 0, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![1, 10],
                vec![10, 1],
                vec![1, 10]
            ],
            5,
            2,
            5
        ),
        5
    );
    assert_eq!(
        min_cost(
            vec![3, 1, 2, 3],
            vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            4,
            3,
            3
        ),
        -1
    );
}
