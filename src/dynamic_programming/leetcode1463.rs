// https://leetcode.com/problems/cherry-pickup-ii/
pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1463() {
    assert_eq!(
        cherry_pickup(vec![
            vec![3, 1, 1],
            vec![2, 5, 1],
            vec![1, 5, 5],
            vec![2, 1, 1]
        ]),
        24
    );
    assert_eq!(
        cherry_pickup(vec![
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![2, 0, 0, 0, 0, 3, 0],
            vec![2, 0, 9, 0, 0, 0, 0],
            vec![0, 3, 0, 5, 4, 0, 0],
            vec![1, 0, 2, 3, 0, 0, 6]
        ]),
        28
    );
    assert_eq!(
        cherry_pickup(vec![
            vec![1, 0, 0, 3],
            vec![0, 0, 0, 3],
            vec![0, 0, 3, 3],
            vec![9, 0, 3, 3]
        ]),
        22
    );
    assert_eq!(cherry_pickup(vec![vec![1, 1], vec![1, 1]]), 4);
}
